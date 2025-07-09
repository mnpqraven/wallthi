{
  config,
  pkgs,
  lib,
  ...
}:
let
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
      config = {
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
}
