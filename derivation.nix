{ stdenv
, rustPlatform
}:

rustPlatform.buildRustPackage rec {
  pname = "xrandr-tool";
  version = "0.1.0";
  src = ./.;
  cargoHash = "sha256-mpslJm/aB59KcTKcn8sowxBX59SympJkTY0KHk0Pynk=";
}
