{
  lib,
  pkg-config,
  rustPlatform,
  gtk3,
  glib,
  ffmpeg,
  opencv4,
  clang,
}:
let
  opencv = (opencv4.override { enableGtk3 = true; });
in
rustPlatform.buildRustPackage (finalAttrs: {
  pname = "slime";
  version = "nightly";
  src = ../.;

  useFetchCargoVendor = true;
  cargoLock.lockFile = ../Cargo.lock;

  nativeBuildInputs = [
    rustPlatform.cargoSetupHook
    rustPlatform.cargoCheckHook
    rustPlatform.bindgenHook

    pkg-config
    gtk3
    glib
    ffmpeg
    clang
  ];

  buildInputs = [
    opencv
  ];

  meta = {
    mainProgram = "slime";
    description = "";
    homepage = "https://github.com/Elecleus/slime";
    license = lib.licenses.gpl3Only;
    maintainers = with lib.maintainers; [ Elecleus ];
    platforms = lib.platforms.unix;
  };
})
