{ pkgs, ... }:
pkgs.rustPlatform.buildRustPackage {
  pname = "xifetch";
  version = "2.0.2";

  src = pkgs.lib.cleanSource ./.;
  cargoLock.lockFile = ./Cargo.lock;
}
