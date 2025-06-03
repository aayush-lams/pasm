{
  lib,
  rustPlatform,
  fetchFromGitHub,
}:

rustPlatform.buildRustPackage rec {
  pname = "pasm";
  version = "1.0.0-alpha";

  src = fetchFromGitHub {
    owner = "Asur-zx";
    repo = "pasm";
    rev = "v${version}";
    hash = "sha256-rsXlcFNWMNznBoidGTUruV1h9YMM+QdkPTfqN2HSB08=";
  };

  cargoHash = "sha256-AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=";

  meta = {
    description = "A minimalistic credential manager";
    homepage = "https://github.com/Asur-zx/pasm/tags";
    license = lib.licenses.unfree; # FIXME: nix-init did not find a license
    maintainers = with lib.maintainers; [ ];
    mainProgram = "pasm";
  };
}
