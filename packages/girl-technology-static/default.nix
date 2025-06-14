{
  pkgs,
  girl-technology,
  ...
}:
pkgs.callPackage ./package.nix { inherit girl-technology; }
