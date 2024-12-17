{ stdenv
, rustPlatform
}:

rustPlatform.buildRustPackage rec {
  pname = "xrandr-tool";
  version = "0.1.0";
  src = ./.;
  cargoHash = "sha256-UyWcAT/269vJ98skK4d1pbC6lO9pu+jx9bmT16MZZf0=";
}
