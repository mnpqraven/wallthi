{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    nixpkgs-mozilla = {
      url = "github:mozilla/nixpkgs-mozilla";
      flake = false;
    };
  };
  outputs =
    {
      flake-utils,
      naersk,
      nixpkgs,
      nixpkgs-mozilla,
      self,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = (import nixpkgs) {
          inherit system;
          overlays = [
            (import nixpkgs-mozilla)
          ];
        };
        toolchain =
          (pkgs.rustChannelOf {
            rustToolchain = ./rust-toolchain.toml;
            sha256 = "sha256-6k1KpO4EeeJE65qomJvmJHcwfcK9LlUUaGeQlhA1zbk=";
          }).rust;
        naersk' = pkgs.callPackage naersk {
          cargo = toolchain;
          rustc = toolchain;
        };
        package = naersk'.buildPackage {
          src = ./.;
        };
      in
      {
        # nix build and nix run
        packages = {
          default = package;
          wallthi = self.packages.${system}.default;
        };

        # nix develop
        devShell = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            rustc
            cargo
          ];
        };
      }
    )
    // {
      homeManagerModules.default = import ./nix/home-manager.nix self;
    };
}
