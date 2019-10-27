with import <nixpkgs> {};
let
  inherit (stdenv) lib;
  moz_overlay = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);
  pkgs = import <nixpkgs> { overlays = [ moz_overlay ]; };
in
  stdenv.mkDerivation {
    name = "rust-env";
    buildInputs = with pkgs; [
        # Additional Dependencies
        (pkgs.latest.rustChannels.beta.rust.override { extensions = [ "rust-src" "rust-std" "rustfmt-preview" "clippy-preview" ]; })
        pkgconfig
        openssl
      ];
    }
