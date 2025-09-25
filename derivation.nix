{ stdenv
, rustPlatform
}:

rustPlatform.buildRustPackage rec {
  pname = "xrandr-tool";
  version = "0.1.0";
  src = ./.;
  cargoHash = "sha256-3cRXduBaAx+BSR+JmPKCeUnIWG9as5bTcqeX5nYpvZ0=";
}
