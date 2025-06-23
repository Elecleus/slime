{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
  };

  outputs =
    {
      flake-parts,
      ...
    }@inputs:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [
        inputs.flake-parts.flakeModules.easyOverlay
      ];

      perSystem =
        {
          config,
          pkgs,
          ...
        }:
        let
          inherit (pkgs)
            callPackage
            ;
          slime = callPackage ./nix/slime.nix { };
        in
        {
          packages.default = slime;
          overlayAttrs = {
            inherit (config.packages) slime;
          };
          packages = {
            inherit slime;
          };
          devShells.default = pkgs.mkShell {
            buildInputs = with pkgs; [
              gtk3
              glib
              ffmpeg
              pkg-config
              (opencv.override { enableGtk3 = true; })

              fish
            ];
            shellHook = ''
              export LIBCLANG_PATH="${pkgs.llvmPackages.libclang.lib}/lib";
              export SHELL=${pkgs.fish}/bin/fish;
              fish
            '';
          };
        };
      systems = [ "x86_64-linux" ];
    };
}
