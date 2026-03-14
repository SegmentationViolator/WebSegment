{
    description = "My personal website";

    inputs = {
        nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
        crane.url = "github:ipetkov/crane";
        flake-utils.url = "github:numtide/flake-utils";
        rust-overlay = {
            url = "github:oxalica/rust-overlay";
            inputs.nixpkgs.follows = "nixpkgs";
        };
    };

    outputs =
        {
            self,
            nixpkgs,
            crane,
            flake-utils,
            rust-overlay,
            ...
        }:
        flake-utils.lib.eachDefaultSystem (
            system:
            let
                pkgs = import nixpkgs {
                    inherit system;
                    overlays = [ (import rust-overlay) ];
                };

                inherit (pkgs) lib;

                rustToolchainFor =
                    p:
                    p.rust-bin.stable.latest.default.override {
                        targets = [ "wasm32-unknown-unknown" ];
                    };
                craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchainFor;

                unfilteredRoot = ./.;
                src = lib.fileset.toSource {
                    root = unfilteredRoot;
                    fileset = lib.fileset.unions [
                        (craneLib.fileset.commonCargoSources unfilteredRoot)
                        (lib.fileset.fromSource (unfilteredRoot + "/assets"))
                        (lib.fileset.fromSource (unfilteredRoot + "/files"))
                        (lib.fileset.fromSource (unfilteredRoot + "/index.html"))
                        (lib.fileset.fromSource (unfilteredRoot + "/posts.json"))
                        (lib.fileset.fromSource (unfilteredRoot + "/stylesheets"))
                    ];
                };

                commonArgs = {
                    inherit src;
                    strictDeps = true;
                    CARGO_BUILD_TARGET = "wasm32-unknown-unknown";
                };

                cargoArtifacts = craneLib.buildDepsOnly (
                    commonArgs
                    // {
                        doCheck = false;
                    }
                );

                app = craneLib.buildTrunkPackage (
                    commonArgs
                    // {
                        inherit cargoArtifacts;
                        trunkExtraBuildArgs = "--filehash=false";
                        wasm-bindgen-cli = pkgs.wasm-bindgen-cli_0_2_100;
                    }
                );
            in
            {
                checks = {
                    app-clippy = craneLib.cargoClippy (
                        commonArgs
                        // {
                            inherit cargoArtifacts;
                            cargoClippyExtraArgs = "--all-targets -- --deny warnings";
                        }
                    );
                };

                packages.default = app;

                devShells.default = craneLib.devShell {
                    checks = self.checks.${system};
                    packages = with pkgs; [
                        trunk
                        dart-sass
                    ];
                };
            }
        );
}
