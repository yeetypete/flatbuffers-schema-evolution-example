{
  projectRootFile = "flake.nix";

  programs.nixfmt.enable = true;
  programs.rustfmt.enable = true;

  # Don't warn on files without a formatter.
  settings.on-unmatched = "info";
}
