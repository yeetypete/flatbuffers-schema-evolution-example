{
  description = "FlatBuffers schema evolution example";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    crane.url = "github:ipetkov/crane";
    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      crane,
      treefmt-nix,
    }:
    let
      forAllSystems =
        f:
        nixpkgs.lib.genAttrs [
          "x86_64-linux"
          "aarch64-linux"
        ] (system: f system nixpkgs.legacyPackages.${system});

      craneLib = forAllSystems (_: crane.mkLib);
      treefmt = forAllSystems (_: pkgs: (treefmt-nix.lib.evalModule pkgs ./nix/treefmt.nix).config.build);
    in
    {
      packages = forAllSystems (
        system: pkgs: {
          default = pkgs.callPackage ./nix/package.nix { craneLib = craneLib.${system}; };
        }
      );

      devShells = forAllSystems (
        system: pkgs: {
          default = craneLib.${system}.devShell {
            checks = self.checks.${system};
            packages = [
              pkgs.rust-analyzer
              treefmt.${system}.wrapper
            ];
          };
        }
      );

      formatter = forAllSystems (system: _: treefmt.${system}.wrapper);

      checks = forAllSystems (
        system: pkgs:
        let
          package = self.packages.${system}.default;
        in
        {
          # Build the crate and run tests.
          inherit package;

          clippy = craneLib.${system}.cargoClippy (
            package.commonArgs
            // {
              inherit (package) cargoArtifacts;
              cargoClippyExtraArgs = "--all-targets -- --deny warnings";
            }
          );

          conform = pkgs.runCommandLocal "flatc-conform" { nativeBuildInputs = [ pkgs.flatbuffers ]; } ''
            flatc --conform ${./schemas/v1/battery_state.fbs} ${./schemas/v2/battery_state.fbs}
            touch $out
          '';

          formatting = treefmt.${system}.check self;
        }
      );
    };
}
