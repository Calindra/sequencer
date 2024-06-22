{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = [
    pkgs.libclang
  ];

  shellHook = ''
    echo "Welcome to the Nix shell with libclang"
  '';
}
