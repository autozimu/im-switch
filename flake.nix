{
  description = "Flake for im-switch";

  inputs = {
    nixpkgs.url = "nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
    }:
    flake-utils.lib.simpleFlake {
      inherit self nixpkgs;
      name = "im-switch";

      shell =
        { pkgs }:
        pkgs.mkShell {
          buildInputs = with pkgs; [
            rustup
            darwin.apple_sdk.frameworks.Carbon
          ];
        };
    };
}
