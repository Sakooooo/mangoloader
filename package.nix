{ rustPlatform }:
rustPlatform.buildRustPackage {
  pname = "mangoloader";
  version = "0.1";
  src = ./.;
  cargoLock.lockFile = ./Cargo.lock;
  meta.mainProgram = "mangoloader";
}
