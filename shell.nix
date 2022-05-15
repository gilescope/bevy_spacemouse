# { pkgs ? import <nixpkgs> {} }:
let
  mozillaOverlay =
    import (builtins.fetchGit {
      url = "https://github.com/mozilla/nixpkgs-mozilla.git";
      rev = "e1f7540fc0a8b989fb8cf701dc4fd7fc76bcf168";
    });
  nixpkgs = import <nixpkgs> { overlays = [ mozillaOverlay ]; };
  rust-stable = with nixpkgs; ((rustChannelOf { channel = "stable"; }).rust.override {
  });
in
with nixpkgs; mkShell {
  buildInputs = [
    udev vulkan-loader
    xlibsWrapper xorg.libXcursor xorg.libXrandr xorg.libXi # To use x11 feature
    libxkbcommon wayland # To use wayland feature
    libspnav # for spacemouse
    rust-stable
  ];
  shellHook = ''export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${pkgs.lib.makeLibraryPath [
    udev vulkan-loader
    libxkbcommon wayland # To use wayland feature
  ]}"'';
}
