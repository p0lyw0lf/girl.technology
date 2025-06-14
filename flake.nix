{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs =
    {
      self,
      flake-utils,
      naersk,
      nixpkgs,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = (import nixpkgs) {
          inherit system;
          overlays = [
            (final: prev: {
              girl-technology = final.pkgs.callPackage ./packages/girl-technology { };
              girl-technology-server = final.pkgs.callPackage ./packages/girl-technology-server { };
              girl-technology-static = final.pkgs.callPackage ./packages/girl-technology-static { };
            })
          ];
        };

        naersk' = pkgs.callPackage naersk { };
        nativeBuildInputs = with pkgs; [ pkg-config ];
        buildInputs = with pkgs; [ openssl ];

      in
      {
        # For `nix build` & `nix run`:
        defaultPackage = naersk'.buildPackage {
          src = ./.;
          inherit nativeBuildInputs;
          inherit buildInputs;
        };

        # For use in infrastructure-y code
        packages = {
          inherit (pkgs) girl-technology-server;
        };

        # For `nix develop`:
        devShell = (import ./shell.nix) {
          inherit pkgs;
          inherit buildInputs;
          inherit nativeBuildInputs;
        };
      }
    );
}
