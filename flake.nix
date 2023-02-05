{
  description = "mondu";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, ... }@inputs:
    let
      overlays = [ (import rust-overlay) ];
    in
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system overlays; };
      in
      with pkgs; rec {
        devShell = mkShell rec {
          name = "mondu";

          nativeBuildInputs = [ pkg-config ];

          buildInputs = [
            (rust-bin.selectLatestNightlyWith
              (toolchain: toolchain.default))
            rust-analyzer
            gdb
            cargo-expand
            cmake
            fontconfig
            libxkbcommon
            libGL
            vulkan-loader
            vulkan-tools
            vulkan-validation-layers
            shaderc
            alsa-lib
            openssl
            systemd # libudev
            # WINIT_UNIX_BACKEND=wayland
            wayland
            # WINIT_UNIX_BACKEND=x11
            xorg.libXcursor
            xorg.libXrandr
            xorg.libXi
            xorg.libX11
          ];

          LD_LIBRARY_PATH = "${lib.makeLibraryPath buildInputs}";
          SHADERC_LIB_DIR = "${shaderc.lib}/lib";
          VK_LAYER_PATH = "${vulkan-validation-layers}/share/vulkan/explicit_layer.d";
        };
      }
    );
}
