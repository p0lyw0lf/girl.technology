{ buildNpmPackage, girl-technology }:
buildNpmPackage {
  pname = "girl-technology-static";
  version = girl-technology.version;

  src = "${girl-technology.src}/static";

  npmDepsHash = "sha256-xpV0z2wym4ZG55eitWBaD4sXgEYXJbpq/+njpYyr/eE=";

  installPhase = ''
    mkdir -p $out/dist
    cp -R dist/* $out/dist
  '';
}
