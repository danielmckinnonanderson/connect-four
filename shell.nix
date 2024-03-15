{ pkgs ? import <nixpkgs> {} }:

with pkgs;

mkShell rec {
  nativeBuildInputs = [
    pkg-config
  ];

  buildInputs = [
    udev alsa-lib vulkan-loader
    xorg.libX11 xorg.libXrandr xorg.libXi xorg.libXcursor
    libxkbcommon
    tiled
  ];

  LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
}

