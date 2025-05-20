{ stdenv
, rustPlatform
}:

rustPlatform.buildRustPackage rec {
  pname = "xrandr-tool";
  version = "0.1.0";
  src = ./.;
  cargoHash = "sha256-IPIKCwzzfVNPVEVhssyaqgpvaldPNTBR0/4sQOA19sA=";
}
