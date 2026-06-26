{
  description = "Canonical Warcraft III game-data extraction crates";

  nixConfig = {
    extra-substituters = ["https://clemenscodes.cachix.org"];
    extra-trusted-public-keys = [
      "clemenscodes.cachix.org-1:yEwW1YgttL2xdsyfFDz/vv8zZRhRGMeDQsKKmtV1N18="
    ];
  };

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane.url = "github:ipetkov/crane";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    fenix,
    crane,
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [
            (final: prev: {
              # Pinned upstream CascLib source. `casclib-rs`'s build script
              # builds CascLib from source via cmake; pointing it at this
              # vendored snapshot makes the extractor build reproducible
              # across machines and keeps it offline-friendly inside Nix.
              casclib = prev.fetchFromGitHub {
                owner = "ladislav-zezula";
                repo = "CascLib";
                rev = "07ab5f37ad282cc101d5c17793c550a0a6d4637f";
                hash = "sha256-E1Z4Y1i3KbMuG17M0L3xCLVVcvAGzF5NWWOadAAw3ZQ=";
              };
            })
          ];
        };

        # Rust toolchain — version, targets, and components declared in
        # rust-toolchain.toml; fenix reads from there.
        rustToolchain = fenix.packages.${system}.fromToolchainFile {
          file = ./rust-toolchain.toml;
          sha256 = "sha256-gh/xTkxKHL4eiRXzWv8KP7vfjSk61Iq48x47BEDFgfk=";
        };

        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

        src = craneLib.cleanCargoSource ./.;

        # `casclib-rs`'s build.rs compiles CascLib from source (cmake) and
        # links zlib. CASCLIB_DIR points it at the pinned source above so the
        # build is hermetic. These are needed for the dependency build too,
        # hence they live in commonArgs.
        commonArgs = {
          inherit src;
          pname = "warcraft-data";
          version = "0.1.0";
          strictDeps = true;
          doCheck = false;
          cargoExtraArgs = "--workspace";
          nativeBuildInputs = with pkgs; [cmake pkg-config];
          buildInputs = [pkgs.zlib];
          CASCLIB_DIR = pkgs.casclib;
        };

        # Cache cargo dependencies separately so a code-only change doesn't
        # rebuild the world (CascLib included).
        cargoArtifacts = craneLib.buildDepsOnly commonArgs;

        # The native binary output: the extractor (regenerates db.rs).
        warcraft-extractor = craneLib.buildPackage (commonArgs
          // {
            inherit cargoArtifacts;
            pname = "warcraft-extractor";
            cargoExtraArgs = "-p warcraft-extractor";
          });

        cargoFmt = craneLib.cargoFmt {inherit src;};

        cargoClippy = craneLib.cargoClippy (commonArgs
          // {
            inherit cargoArtifacts;
            cargoClippyExtraArgs = "-- -D warnings";
          });

        cargoTest = craneLib.cargoTest (commonArgs // {inherit cargoArtifacts;});
      in {
        formatter = pkgs.alejandra;

        packages = {
          default = warcraft-extractor;
          inherit warcraft-extractor cargoArtifacts;
        };

        checks = {
          inherit warcraft-extractor cargoFmt cargoClippy cargoTest;
        };

        # `nix run .#extract -- --casc /path/to/Warcraft\ III/Data` rebuilds
        # crates/warcraft-database/src/db.rs from CASC. Delegates to the dev
        # shell so build-time linker flags (zlib, libstdc++) are wired up the
        # same way they are interactively.
        apps.extract = let
          extractApp = pkgs.writeShellApplication {
            name = "warcraft-extract";
            runtimeInputs = [pkgs.nix];
            text = ''
              exec nix develop . --command \
                cargo run -p warcraft-extractor -- "$@"
            '';
          };
        in {
          type = "app";
          program = "${extractApp}/bin/warcraft-extract";
          meta = {
            description = "Regenerate db.rs from a Warcraft III CASC archive";
            mainProgram = "warcraft-extract";
          };
        };

        devShells.default = pkgs.mkShell {
          inputsFrom = [warcraft-extractor];
          packages = with pkgs; [
            rustToolchain
            cargo-watch
            cargo-edit
            cargo-nextest
            taplo
            alejandra
            nil
            cmake
            pkg-config
            zlib
          ];

          # `casclib-rs`' build script reads CASCLIB_DIR to locate the CascLib
          # source it compiles. Pointing it at the pinned overlay attribute
          # makes the extractor build reproducible without network fetches.
          CASCLIB_DIR = pkgs.casclib;

          # Runtime linking for the extractor binary: zlib is dlopened by the
          # freshly-built CascLib, gcc.cc.lib provides libstdc++ for its C++.
          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath (with pkgs; [gcc.cc.lib zlib]);

          shellHook = ''
            echo ""
            echo "  warcraft-data — extraction crates dev shell"
            echo ""
            echo "  Regenerate db.rs:"
            echo "    cargo run -p warcraft-extractor -- --casc \"\$W3_CASC\""
            echo "    cargo fmt -p warcraft-database"
            echo ""
            echo "  Set W3_CASC to your Warcraft III install's Data/ dir."
            echo ""
            if [ -z "''${W3_CASC:-}" ]; then
              for candidate in \
                "''${WINEPREFIX:-$HOME/.wine}/drive_c/Program Files (x86)/Warcraft III/Data" \
                "$HOME/Games/W3Champions/drive_c/Program Files (x86)/Warcraft III/Data"; do
                if [ -d "$candidate" ]; then
                  export W3_CASC="$candidate"
                  echo "  W3_CASC auto-detected: $W3_CASC"
                  echo ""
                  break
                fi
              done
            fi
          '';
        };
      }
    );
}
