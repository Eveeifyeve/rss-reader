{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    agenix-shell.url = "github:aciceri/agenix-shell";
  };

  outputs = inputs@{ self, nixpkgs, flake-parts, agenix-shell, ... }:
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
        devShells.default = pkgs.mkShell
        {
          packages = with pkgs; [
            rustc
            cargo
            clippy
            python3
            darwin.libobjc
          ];

        };
      };
    };
}