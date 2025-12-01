{
  description = "Yet Another Mid Ahh Filehost";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/release-25.05";

    crane.url = "github:ipetkov/crane";

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    inputs@{
      self,
      nixpkgs,
      crane,
      fenix,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ fenix.overlays.default ];
        };

        inherit (pkgs) lib;

        toolchain = pkgs.fenix.fromToolchainFile {
          file = ./rust-toolchain.toml;
          sha256 = "sha256-SDu4snEWjuZU475PERvu+iO50Mi39KVjqCeJeNvpguU=";
        };

        craneLib = (crane.mkLib pkgs).overrideToolchain toolchain;
        src = ./.;

        commonArgs = { inherit src; };

        cargoArtifacts = craneLib.buildDepsOnly commonArgs;
        package =
          {
            craneLib,
            lib,
            src,
          }:
          craneLib.buildPackage {
            inherit cargoArtifacts src;
            doCheck = false;
          };
      in
      {

        packages = {
          inherit package;
          default = package;
          checks = {
            clippy = craneLib.cargoClippy (
              commonArgs
              // {
                inherit cargoArtifacts;
              }
            );

            fmt = craneLib.cargoFmt {
              inherit src;
            };

            doc = craneLib.cargoDoc (
              commonArgs
              // {
                inherit cargoArtifacts;
              }
            );

            nextest = craneLib.cargoNextest (
              commonArgs
              // {
                inherit cargoArtifacts;
                partitions = 1;
                partitionType = "count";
              }
            );
          };
        };

        devShells.default = pkgs.mkShell {
          nativeBuildInputs = [ toolchain ];
        };

        formatter = pkgs.nixfmt-tree;
      }
    );
}
