{
  description = "easy-template";

  # Flake inputs
  inputs = {
    nixpkgs.url = "https://flakehub.com/f/NixOS/nixpkgs/0.1";
    fenix = {
      url = "https://flakehub.com/f/nix-community/fenix/0.1";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane.url = "https://flakehub.com/f/ipetkov/crane/0";
  };

  # Flake outputs
  outputs = inputs:
    let
      lastModifiedDate = inputs.self.lastModifiedDate or inputs.self.lastModified or "19700101";
      version = "${builtins.substring 0 8 lastModifiedDate}-${inputs.self.shortRev or "dirty"}";
      meta = (builtins.fromTOML (builtins.readFile ./Cargo.toml)).package;

      # The systems supported for this flake
      supportedSystems = [
        "x86_64-linux" # 64-bit Intel/AMD Linux
        "aarch64-linux" # 64-bit ARM Linux
        "aarch64-darwin" # 64-bit ARM macOS
      ];

      # Helper to provide system-specific attributes
      forEachSupportedSystem = f: inputs.nixpkgs.lib.genAttrs supportedSystems (system: f {
        pkgs = import inputs.nixpkgs {
          inherit system;
          overlays = [
            inputs.fenix.overlays.default
            inputs.self.overlays.default
          ];
        };
      });
    in
    {
      devShells = forEachSupportedSystem ({ pkgs }: {
        default = pkgs.mkShell {
          # The Nix packages provided in the environment
          # Add any you need here
          packages = with pkgs; [
            rustToolchain
            cargo-edit
            cargo-watch
            bacon
          ];
        };
      });

      packages = forEachSupportedSystem ({ pkgs }: {
        default = pkgs.easy-template;
      });

      overlays.default = final: prev:
        let
          system = final.hostPlatform.system;
          rustToolchain = with inputs.fenix.packages.${system};
            combine (with stable; [
              cargo
              clippy
              rustc
              rustfmt
              rust-src
            ]);

          easy-template = ((inputs.crane.mkLib final).overrideToolchain rustToolchain).buildPackage ({
            pname = meta.name;
            inherit (meta) version;
            src = builtins.path { name = "easy-template-source"; path = ./.; };
          });
        in
        { inherit easy-template rustToolchain; };
    };
}
