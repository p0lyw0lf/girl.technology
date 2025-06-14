{
  pkgs,
  ...
}:
pkgs.callPackage ./package.nix {
  inherit (pkgs) girl-technology girl-technology-static;
  diesel-cli = pkgs.diesel-cli.override {
    sqliteSupport = false;
    postgresqlSupport = true;
    mysqlSupport = false;
  };
}
