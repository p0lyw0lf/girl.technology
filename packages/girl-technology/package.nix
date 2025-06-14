{
  fetchFromGitHub,
  rustPlatform,
}:
let
  version = "0.4.1";
in
rustPlatform.buildRustPackage {
  pname = "girl_technology";
  inherit version;

  src = fetchFromGitHub {
    owner = "p0lyw0lf";
    repo = "girl.technology";
    tag = "v${version}";
    hash = "sha256-XYk2SZ2E96eGJ3miM7ktsWEQbJLU2W6UW92AILx3gkI=";
  };

  cargoHash = "sha256-90dbFYXdPQxP07Z0aZ5++1KgUIkPxTVPyi255cyo2nA=";
}
