{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs = { self, flake-utils, naersk, nixpkgs }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = (import nixpkgs) {
          inherit system;
        };

        naersk' = pkgs.callPackage naersk {};
        nativeBuildInputs = with pkgs; [ pkg-config ];
        buildInputs = with pkgs; [ openssl ];

      in rec {
        # For `nix build` & `nix run`:
        defaultPackage = naersk'.buildPackage {
          src = ./.;
          inherit nativeBuildInputs;
          inherit buildInputs;
        };

        # For `nix develop`:
        devShell = pkgs.mkShell {
          packages = with pkgs;
            [
              cargo-watch
              (diesel-cli.override {
                sqliteSupport = false;
                postgresqlSupport = true;
                mysqlSupport = false;
              })
              nodejs_20
            ];
          nativeBuildInputs = nativeBuildInputs ++ (with pkgs; [ rustc cargo ]);
          inherit buildInputs;
        };
      }
    );
}
