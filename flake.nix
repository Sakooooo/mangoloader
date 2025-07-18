{
  description = "Mangoloader nix flake because get it nix flake nix flake nix reproduce";

  inputs = {nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";};

  outputs = {
    self,
    nixpkgs,
  }: let
    inherit (nixpkgs) lib;
    inherit (builtins) attrValues;
    eachSystem = f:
      lib.genAttrs ["x86_64-linux" "aarch64-linux"]
      (system: f nixpkgs.legacyPackages.${system});
  in {
    # packages = eachSystem (pkgs: rec {
    #   default = mangoloader;
    #   mangoloader = pkgs.callPackage ./package.nix { };
    # });

    devShells = eachSystem (pkgs: {
      default = pkgs.mkShell {
        packages = attrValues {
          inherit
            (pkgs)
            cargo
            rustc
            rust-analyzer
            rustfmt
            pkg-config
            sqlite-interactive
            openssl
            vscode-langservers-extracted
            sqlx-cli
            ;
          inherit (pkgs.nodePackages) prettier;
        };
      };
    });

    formatter = eachSystem (pkgs: pkgs.alejandra);
  };
}
