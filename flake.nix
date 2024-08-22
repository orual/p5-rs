{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    devshell.url = "github:numtide/devshell";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, flake-utils, naersk, nixpkgs, rust-overlay, devshell, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = (import nixpkgs) {
          inherit system overlays;
        };

        extensions = [
          "rust-src" # for rust-analyzer
          "rust-analyzer"
        ];

        targets = [
          "x86_64-unknown-linux-gnu"
          #"armv7-unknown-linux-gnueabihf"
          "aarch64-unknown-linux-gnu"
        ];

        toolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = extensions;
          targets = targets;
        };

        naersk' = naersk.lib.${system}.override {
          cargo = toolchain;
          rustc = toolchain;
        };

        cargoConfig = { };

        nativeBuildInputs = with pkgs; [
          pkg-config
          stdenv.cc
        ];
        buildInputs = with pkgs; [
          pkg-config
          stdenv.cc
          udev
          alsa-lib
          vulkan-loader
          xorg.libX11
          xorg.libXcursor
          xorg.libXi
          xorg.libXrandr # To use the x11 feature
          libxkbcommon
          wayland # To use the wayland feature
        ];

      in
      rec {
        # For `nix build` & `nix run`:
        defaultPackage = naersk'.buildPackage {
          src = ./.;
          buildInputs = buildInputs;
          nativeBuildInputs = nativeBuildInputs;
        };

        # Run `nix build .#test` to run tests
        test = naersk'.buildPackage {
          src = ./.;
          mode = "test";
          buildInputs = buildInputs;
          nativeBuildInputs = nativeBuildInputs;
          #cargoTestOptions = [ ''cargo_test_options="$cargo_test_options --lib"'' ];
        };

        # Run `nix build .#check` to check code
        check = naersk'.buildPackage {
          src = ./.;
          mode = "check";
          buildInputs = buildInputs;
          nativeBuildInputs = nativeBuildInputs;
        };

        packages.devshell = self.outputs.devShells.${system}.default;

        devShells.default =
          let
            pkgs = import nixpkgs {
              inherit system;
              overlays = [ devshell.overlays.default ];
            };
          in
          pkgs.devshell.mkShell ({ config, lib, ... }: {
            name = "p5rs";
            env = [
              {
                name = "LD_LIBRARY_PATH";
                value = lib.makeLibraryPath buildInputs;
              }
            ];

            packages = [
              defaultPackage
              toolchain
              pkgs.cargo-udeps
            ];

            commands = [
              {
                name = "greet";
                command = ''
                  printf -- 'Hello, %s!\n' "''${1:-world}"
                '';
              }
            ];
          } // cargoConfig);
      }
    );
}
