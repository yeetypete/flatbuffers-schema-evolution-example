{
  projectRootFile = "flake.nix";

  programs.nixfmt.enable = true;
  programs.rumdl-format.enable = true;
  programs.rustfmt.enable = true;

  # Wrap text at 80 columns, but skip code blocks.
  settings.formatter.rumdl-format.options = [
    "--config"
    "MD013.reflow = true"
    "--config"
    "MD013.code-blocks = false"
  ];

  # Don't warn on files without a formatter.
  settings.on-unmatched = "info";
}
