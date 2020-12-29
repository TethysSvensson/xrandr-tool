{ stdenv
, rustPlatform
}:

rustPlatform.buildRustPackage rec {
  pname = "xrandr-tool";
  version = "0.1.0";
  src = ./.;
  cargoSha256 = "0m93zybw31zh2qg0jyca9k1jch88vacyv93jglbbqjr3dying24q";
}
