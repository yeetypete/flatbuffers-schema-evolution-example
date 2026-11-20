{
  lib,
  craneLib,
  flatbuffers,
}:
let
  commonArgs = {
    src = lib.fileset.toSource {
      root = ../.;
      fileset = lib.fileset.unions [
        (craneLib.fileset.commonCargoSources ../.)
        ../schemas
      ];
    };
    strictDeps = true;

    # build.rs runs flatc.
    nativeBuildInputs = [ flatbuffers ];
  };
  cargoArtifacts = craneLib.buildDepsOnly commonArgs;
in
craneLib.buildPackage (
  commonArgs
  // {
    inherit cargoArtifacts;
    passthru = { inherit commonArgs cargoArtifacts; };
    meta.mainProgram = "flatbuffers-schema-evolution-example";
  }
)
