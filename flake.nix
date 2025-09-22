{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs =
    {
      flake-utils,
      naersk,
      nixpkgs,
      rust-overlay,
      self,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = (import nixpkgs) {
          inherit system;
          overlays = [
            (import rust-overlay)
          ];
        };
        toolchain = pkgs.rust-bin.fromRustupToolchain (
          (builtins.fromTOML (builtins.readFile ./rust-toolchain.toml)).toolchain // { "components" = [ ]; }
        );
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
