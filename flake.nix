{
  description = "Description for the project";

  inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.05";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    inputs@{ flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [
        # To import a flake module
        # 1. Add foo to inputs
        # 2. Add foo as a parameter to the outputs function
        # 3. Add here: foo.flakeModule

      ];
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "aarch64-darwin"
        "x86_64-darwin"
      ];
      perSystem =
        {
          config,
          self',
          inputs',
          pkgs,
          system,
          ...
        }:
        let
          rust-stuff = with inputs'.fenix.packages; [
              (combine [
                stable.rustc
                targets.x86_64-unknown-none.stable.completeToolchain
                stable.rust-analyzer
              ])
          ];
        in

        {
          # Per-system attributes can be defined here. The self' and inputs'
          # module parameters provide easy access to attributes of the same
          # system.

          # Equivalent to  inputs'.nixpkgs.legacyPackages.hello;
          devShells.default = pkgs.mkShell {
            buildInputs = with pkgs; [
              gnumake
              autoconf
              autoconf.doc
              m4
              automake
              gcc
              gdb
              linuxHeaders
              act
              moreutils
              rust-stuff
            ];

            # R = "${inputs'.fenix.packages.targets.x86_64-unknown-none.stable.completeToolchain}";
            shellHook = ''
              #export PATH=~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin:$PATH
            '';

          };
        };
      flake = {
        # The usual flake attributes can be defined here, including system-
        # agnostic ones like nixosModule and system-enumerating ones, although
        # those are more easily expressed in perSystem.

      };
    };
}
