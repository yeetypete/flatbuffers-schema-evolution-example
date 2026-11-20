{
  description = "FlatBuffers schema evolution example";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      treefmt-nix,
    }:
    let
      forAllSystems =
        f:
        nixpkgs.lib.genAttrs [
          "x86_64-linux"
          "aarch64-linux"
        ] (system: f system nixpkgs.legacyPackages.${system});

      treefmt = forAllSystems (_: pkgs: (treefmt-nix.lib.evalModule pkgs ./nix/treefmt.nix).config.build);
    in
    {
      devShells = forAllSystems (
        system: pkgs: {
          default = pkgs.mkShell {
            packages = [
              pkgs.cargo
              pkgs.clippy
              pkgs.flatbuffers
              pkgs.rust-analyzer
              pkgs.rustc
              pkgs.rustfmt
              treefmt.${system}.wrapper
            ];
          };
        }
      );

      formatter = forAllSystems (system: _: treefmt.${system}.wrapper);

      checks = forAllSystems (
        system: _: {
          formatting = treefmt.${system}.check self;
        }
      );
    };
}
