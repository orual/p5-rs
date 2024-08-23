{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { flake-utils, naersk, nixpkgs, rust-overlay, ... }:
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

        devShell = pkgs.mkShell
          (
            {
              inputsFrom = [ defaultPackage ];
              buildInputs = buildInputs;
              nativeBuildInputs = nativeBuildInputs;
              packages = [
                defaultPackage
              ];
            } // cargoConfig
          );
      }
    );
}
