{
  pkgs ? import <nixpkgs> { },
  ...
}:
pkgs.mkShell {
  strictDeps = true;

  nativeBuildInputs = with pkgs; [
    cargo
    clippy
    just
    rustc
    rustfmt
  ];
}
