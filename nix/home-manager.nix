self:
{
  config,
  pkgs,
  lib,
  ...
}:
let
  # ref https://github.com/NixOS/nixpkgs/blob/4ffc4dc91838df228c8214162c106c24ec8fe03f/nixos/modules/programs/starship.nix
  cfg = config.programs.wallthi;

  inherit (pkgs.stdenv.hostPlatform) system;
  package = self.packages.${system}.default;

  tomlFormat = pkgs.formats.toml { };

  settingsFileToml = tomlFormat.generate "config.toml" cfg.settings;
in
with lib;
{
  options.programs.wallthi = {
    enable = lib.mkEnableOption "swww wrapper";

    package = lib.mkOption {
      type = lib.types.package;
      default = package;
      defaultText = lib.literalExpression "pkgs.wallthi";
      description = ''
        Package to use. Set to `null` to use the default package.
      '';
    };

    settings = {
      general = {
        duration = lib.mkOption {
          default = 60;
          type = lib.types.int;
          description = "duration of each monitor";
        };
        path = lib.mkOption {
          default = [ ];
          type = lib.types.listOf lib.types.str;
          description = "dirs containing wallpapers";
        };
        path_vertical = lib.mkOption {
          default = [ ];
          type = lib.types.listOf lib.types.str;
          description = "dirs containing wallpapers";
        };
      };

      monitor = lib.mkOption {
        default = { };
        type =
          with types;
          attrsOf (
            submodule (
              { name, ... }:
              {
                options = {
                  resolution = mkOption { type = str; };
                  transform = mkOption {
                    type = nullOr int;
                    default = 0;
                  };
                  vertical = mkOption {
                    type = nullOr bool;
                    default = false;
                  };
                };
              }
            )
          );
        example = lib.literalExpression ''
          {
            "HDMI-A-1" = {
              resolution = "1920x1080";
              transform = 90;
              vertical = true;
            };
          }
        '';
        description = "map containing monitor name and config as key-value";
      };
    };
  };

  config = lib.mkIf cfg.enable {
    home.packages = [ cfg.package ];
    xdg.configFile."wallthi/config.conf" = {
      source = settingsFileToml;
    };
  };
}
