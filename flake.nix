{
  description = "A tweaking tool for the COSMIC desktop";

  inputs = {
    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    inputs:
    let
      inherit (inputs.nixpkgs) lib;
    in
    inputs.flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [
        "aarch64-linux"
        "x86_64-linux"
      ];

      imports = lib.optionals (inputs.treefmt-nix ? flakeModule) [ inputs.treefmt-nix.flakeModule ];

      perSystem =
        { pkgs, self', ... }:
        {
          devShells.default = import ./shell.nix { inherit pkgs; };

          packages = {
            default = self'.packages.cosmic-ext-tweaks;
            cosmic-ext-tweaks = import ./. { inherit pkgs; };
          };
        }
        // lib.optionalAttrs (inputs.treefmt-nix ? flakeModule) {
          treefmt.config = {
            flakeCheck = true;
            projectRootFile = "flake.nix";

            programs = {
              nixfmt = {
                enable = true;
                package = pkgs.nixfmt-rfc-style;
              };
              rustfmt = {
                enable = true;
                package = pkgs.rustfmt;
              };
            };
          };
        };
    };
}
