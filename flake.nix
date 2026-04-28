{
  description = "rubbish devshell and package";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
      in {
        devShells.default = pkgs.mkShell {
          name = "rubbish-devshell";

          packages = with pkgs; [
            go
            gopls
            gotools
            delve
            just
          ];
        };

        packages.rubbish = pkgs.buildGoModule {
          pname = "rubbish";
          version = "2026.04.28-a";

          src = self;

          vendorHash = "sha256-TziD3Mq0e/+zeiRW/X3wOt/91V8sQsYhSFHf8qj0gmU=";

          subPackages = [ "." ];
          ldflags = [ "-s" "-w" ];

          meta = with pkgs.lib; {
            description = "A CLI file trash tool, a replacement for rm";
            license = licenses.mit;
            platforms = platforms.all;
          };
        };

        apps.rubbish = {
          type = "app";
          program = "${self.packages.${system}.rubbish}/bin/rubbish";
        };
      });
}
