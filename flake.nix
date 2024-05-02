{
  description = "A simple Wordle clone with a CLI version and a web app";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.11";
    flake-parts.url = "github:hercules-ci/flake-parts";

    pre-commit-hooks = {
      url = "github:cachix/pre-commit-hooks.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs @ {flake-parts, ...}:
    flake-parts.lib.mkFlake {inherit inputs;} {
      imports = [
        inputs.pre-commit-hooks.flakeModule
      ];

      systems = ["x86_64-linux" "aarch64-linux"];
      perSystem = {
        config,
        system,
        ...
      }: let
        pkgs = import inputs.nixpkgs {
          inherit system;
          overlays = [(import inputs.rust-overlay)];
        };

        rustToolchain = pkgs.rust-bin.stable.latest.default;

        craneLib = (inputs.crane.mkLib pkgs).overrideToolchain rustToolchain;
        src = pkgs.lib.cleanSourceWith {
          src = ./.;
          filter = path: type:
            (pkgs.lib.hasSuffix "\.html" path)
            || (pkgs.lib.hasSuffix "\.scss" path)
            || (craneLib.filterCargoSources path type);
        };

        commonArgs = {
          inherit src;
          strictDeps = true;
        };

        cargoArtifacts = craneLib.buildDepsOnly commonArgs;

        individualCrateArgs =
          commonArgs
          // {
            inherit cargoArtifacts;
            inherit (craneLib.crateNameFromCargoToml {inherit src;}) version;
          };

        trunkPreBuildTools = with pkgs.nodePackages; [
          autoprefixer
          postcss
          postcss-cli
          sass
        ];
      in rec {
        devShells.default = pkgs.mkShell {
          nativeBuildInputs =
            [
              (rustToolchain.override {
                extensions = ["rust-analyzer" "rust-src" "rust-std"];
              })
              pkgs.cargo-nextest
            ]
            ++ trunkPreBuildTools;
          shellHook = ''
            ${config.pre-commit.installationScript}
          '';
        };

        # See https://flake.parts/options/pre-commit-hooks-nix and
        # https://github.com/cachix/git-hooks.nix/blob/master/modules/hooks.nix
        # for all the available hooks and options
        pre-commit.settings.hooks = {
          check-added-large-files.enable = true;
          check-merge-conflicts.enable = true;
          check-toml.enable = true;
          check-vcs-permalinks.enable = true;
          check-yaml.enable = true;
          end-of-file-fixer.enable = true;
          trim-trailing-whitespace.enable = true;

          rustfmt = {
            enable = true;
            packageOverrides = {
              cargo = rustToolchain;
              rustfmt = rustToolchain;
            };
          };
        };

        checks = {
          inherit (packages) cli doc web;

          clippy = craneLib.cargoClippy (commonArgs
            // {
              inherit cargoArtifacts;
              cargoClippyExtraArgs = "--all-targets -- --deny warnings";
            });

          fmt = craneLib.cargoFmt {
            inherit src;
          };

          nextest = craneLib.cargoNextest (commonArgs
            // {
              inherit cargoArtifacts;
              partitions = 1;
              partitionType = "count";
            });
        };

        packages = {
          cli = craneLib.buildPackage (individualCrateArgs
            // {
              pname = "wordle-cli";
              cargoExtraArgs = "--package=wordle-cli";
            });

          doc = craneLib.cargoDoc (commonArgs
            // {
              inherit cargoArtifacts;
              cargoDocExtraArgs = "--no-deps --document-private-items --workspace";
              RUSTDOCFLAGS = "--deny warnings";
            });

          web = let
            rustToolchainWasm = rustToolchain.override {
              targets = ["wasm32-unknown-unknown"];
            };
            craneLibTrunk =
              ((inputs.crane.mkLib pkgs).overrideToolchain rustToolchainWasm)
              .overrideScope (_: _: {inherit (pkgs) wasm-bindgen-cli;});
          in
            craneLibTrunk.buildTrunkPackage (commonArgs
              // {
                pname = "wordle-web";
                inherit cargoArtifacts;

                trunkIndexPath = "web/index.html";
                cargoExtraArgs = "--package=wordle-web";
                CARGO_BUILD_TARGET = "wasm32-unknown-unknown";
                inherit (pkgs) wasm-bindgen-cli;

                nativeBuildInputs = trunkPreBuildTools;
                preBuild = ''
                  sass web/main.scss web/sass.css
                  mv web/sass.css web/_main.css
                  # TODO: Use postcss properly. Why doesn't it recognise autoprefixer?
                  # postcss --use autoprefixer -o web/_main.css web/sass.css
                '';
              });
        };
      };
    };
}
