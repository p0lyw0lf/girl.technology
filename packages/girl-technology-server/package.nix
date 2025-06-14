{
  stdenvNoCC,
  bash,
  diesel-cli,
  girl-technology,
  girl-technology-static,
}:
stdenvNoCC.mkDerivation (finalAttrs: {
  pname = "girl_technology_server";
  version = girl-technology.version;
  src = girl-technology.src;

  installPhase = ''
    mkdir -p $out/assets
    cp -R assets/* $out/assets

    mkdir -p $out/migrations
    cp -R migrations/* $out/migrations

    mkdir -p $out/static/dist
    cp -R ${girl-technology-static}/dist/* $out/static/dist

    mkdir -p $out/templates
    cp -R templates/* $out/templates

    cp diesel.toml $out/diesel.toml

    mkdir -p $out/bin
    cat <<EOF > $out/bin/girl_technology_server
    #!${bash}/bin/bash
    set -euo
    ${diesel-cli}/bin/diesel setup
    ${girl-technology}/bin/girl_technology
    EOF
    chmod +x $out/bin/girl_technology_server
  '';
})
