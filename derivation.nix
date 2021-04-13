{ stdenv
, rustPlatform
}:

rustPlatform.buildRustPackage rec {
  pname = "xrandr-tool";
  version = "0.1.0";
  src = ./.;
  cargoSha256 = "12d2n0k10yy6dgi5i55ab41zva3ccpzzkj0qc4x43kqpxsb02piv";
}
