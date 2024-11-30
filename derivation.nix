{ stdenv
, rustPlatform
}:

rustPlatform.buildRustPackage rec {
  pname = "xrandr-tool";
  version = "0.1.0";
  src = ./.;
  cargoHash = "sha256-5KVlZlcgP1B2SmfrVCIQXOHw/MEKuFW8JrAEwEEYuWI=";
}
