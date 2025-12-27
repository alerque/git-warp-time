# SPDX-FileCopyrightText: © 2021 Caleb Maclennan <caleb@alerque.com>
# SPDX-License-Identifier: GPL-3.0-only
{
  description = "Rust library and CLI utility to reset file timestamps to repo state";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    naersk.url = "github:nix-community/naersk";
    naersk.inputs.nixpkgs.follows = "nixpkgs";
  };
  outputs = { self, nixpkgs, naersk }:
    let
      cargoToml = (builtins.fromTOML (builtins.readFile ./Cargo.toml));
      supportedSystems = [ "x86_64-linux" "aarch64-linux" "x86_64-darwin" ];
      forAllSystems = f: nixpkgs.lib.genAttrs supportedSystems (system: f system);
      pkgsFor = forAllSystems (system:
        import nixpkgs {
          inherit system;
          overlays = [ self.overlays.default ];
        });
    in
    {
      overlays.default = final: prev: {
        "${cargoToml.package.name}" = final.callPackage ./. { inherit naersk; };
      };
      packages = forAllSystems (system: {
        default = pkgsFor.${system}.${cargoToml.package.name};
        ${cargoToml.package.name} = pkgsFor.${system}.${cargoToml.package.name};
      });
      devShells = forAllSystems (system:
        let
          pkgs = pkgsFor.${system};
        in
        {
          default = pkgs.mkShell {
            inputsFrom = [ pkgs.${cargoToml.package.name} ];
            buildInputs = with pkgs; [
              libgit2
            ];
          };
        });
    };
}
