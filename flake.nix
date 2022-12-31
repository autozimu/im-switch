{
  description = "Flake for im-switch";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-22.11";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.simpleFlake {
      inherit self nixpkgs;
      name = "im-switch";

      shell = { pkgs }: pkgs.mkShell {
        buildInputs = with pkgs; [
          rustup
        ] ++ lib.optionals stdenvNoCC.isDarwin [
          libiconv
          darwin.apple_sdk.frameworks.Carbon
        ];
      };
    };
}
