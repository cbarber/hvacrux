{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
  };
  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem
      (system:
        let
          overlays = [ (import rust-overlay) ];
          pkgs = import nixpkgs {
            inherit system overlays;
          };
          rustToolchain = pkgs.pkgsBuildHost.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
          nativeBuildInputs = with pkgs; [ rustToolchain pkg-config ];
          cargoLeptos = pkgs.rustPlatform.buildRustPackage rec {
            pname = "cargo-leptos";
            version = "0.2.16";

            src = pkgs.fetchFromGitHub {
              owner = "leptos-rs";
              repo = pname;
              rev = "07db33b6dc1f5e20521b59744f3b4fea361250f0";
              hash = "sha256-yDBVo3GarKvcuMX7mxdxx0SJSJ5VOX3bUx6XmYxLfq4=";
            };

            cargoHash = "sha256-DZbZ3SHGWvje0gEqlx2mdLvCR4U3Xzkp8gS9FIbxW6g=";

            doCheck = false;
          };
          buildInputs = with pkgs; [ rust-analyzer cargoLeptos openssl mold clang ];
        in
        with pkgs;
        {
          devShells.default = mkShell {
            inherit buildInputs nativeBuildInputs;
          };
        }
      );
}
