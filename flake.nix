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

          daisyui = pkgs.buildNpmPackage rec {
            pname = "daisyui";
            version = "4.6.3";

            src = pkgs.fetchFromGitHub {
              owner = "saadeghi";
              repo = pname;
              rev = "v${version}";
              hash = "sha256-O1YZF2mMNWnoj6sRrbQKJBTqlQ+NIcpZf0kawDDeVxM=";
            };

            npmDepsHash = "sha256-Y+poto5nGcFgBHcCl6MVwUeBMAnKv+pgxtODXnCSd3U=";

            # use generated package-lock.json as upstream does not provide one
            postPatch = ''
              cp ${./hvacrux-leptos/daisyui-package-lock.json} ./package-lock.json
            '';

            # The prepack script runs the build script, which we'd rather do in the build phase.
            npmPackFlags = ["--ignore-scripts"];

            NODE_OPTIONS = "--openssl-legacy-provider";
          };

          tailwindCss = pkgs.nodePackages.tailwindcss.overrideAttrs (oa: {
            plugins = [
              pkgs.nodePackages."@tailwindcss/aspect-ratio"
              pkgs.nodePackages."@tailwindcss/forms"
              pkgs.nodePackages."@tailwindcss/language-server"
              pkgs.nodePackages."@tailwindcss/line-clamp"
              pkgs.nodePackages."@tailwindcss/typography"
              daisyui
            ];
          });

          nativeBuildInputs = with pkgs; [ rustToolchain pkg-config rust-analyzer tailwindCss ];

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

          buildInputs = with pkgs; [ cargoLeptos openssl mold clang ];
        in
        with pkgs;
        {
          devShells.default = mkShell {
            inherit buildInputs nativeBuildInputs;
          };
        }
      );
}
