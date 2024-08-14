{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = [
    pkgs.libclang
    pkgs.libgcc
    pkgs.gcc
  ];

  shellHook = ''
    echo "Welcome to the Nix shell"
    export GCC_PATH="${pkgs.libgcc}"
  '';
}
