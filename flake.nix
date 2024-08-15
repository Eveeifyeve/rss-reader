{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
  };

  outputs = inputs@{ self, nixpkgs, flake-parts, ... }:
    flake-parts.lib.mkFlake {inherit inputs;} {
      systems = nixpkgs.lib.systems.flakeExposed;
      perSystem = {
        pkgs,
        config,
        lib,
        ...
      }: 
      let
        inherit (pkgs.darwin.apple_sdk.frameworks) CoreFoundation;
      in
      {
        devShells.default = pkgs.mkShell.override {
          stdenv = pkgs.llvmPackages.stdenv;
        }
        {
          packages = with pkgs; [
            rustc
            rustfmt
            cargo
            clippy
            libiconv
            darwin.libobjc
          ];
        };
      };
    };
}