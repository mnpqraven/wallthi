{
  config,
  pkgs,
  lib,
  ...
}:
let
  # ref https://github.com/NixOS/nixpkgs/blob/4ffc4dc91838df228c8214162c106c24ec8fe03f/nixos/modules/programs/starship.nix
  cfg = config.programs.wallthi;
  settingsFormat = pkgs.formats.toml { };

  # place this inside xdg config
  settingsFile = settingsFormat.generate "config.toml" cfg.settings;

  monitorConfigType = with lib; {
    resolution = types.string;
    transform = nullOr types.int;
    vertical = nullOr types.bool;
  };
in
{
  options = {
    programs.wallthi = {
      enable = lib.mkEnableOption "swww wrapper";
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
          pathVertical = lib.mkOption {
            default = [ ];
            type = lib.types.listOf lib.types.str;
            description = "dirs containing wallpapers";
          };
        };

        monitor = lib.mkOption {
          default = { };
          type = lib.types.attrsOf monitorConfigType;
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
  };

  xdg.configFile."wallthi/config.conf" = {
    source = settingsFile;
  };
}
