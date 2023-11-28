{ lib
, rustPlatform
, fetchFromGitHub
}:

rustPlatform.buildRustPackage rec {
  pname = "prompt-shortener";
  version = "0.1.2";

  src = fetchFromGitHub {
    owner = "camper0008";
    repo = "prompt-shortener";
    rev = "1447cffb230ab3983a3e5839ee340553cb13ee65";
    hash = "sha256-RrVttmAJrfz4gTtL1MuU9QDNIkhVD3hd3t+7t+BfqMw=";
  };

  cargoHash = "sha256-6gDDmIYrWc9MkbQ+ExjNM5BLpEbDAYrhruTJ7bXCyh0=";

  meta = with lib; {
    description = "A prompt workdir shortener";
    homepage = "https://github.com/camper0008/prompt-shortener";
    license = licenses.gpl3Only;
    maintainers = with maintainers; [ ];
  };
}
