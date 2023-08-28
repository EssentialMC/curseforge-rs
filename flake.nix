{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    systems.url = "github:nix-systems/default";
    rust-overlay.url = "github:oxalica/rust-overlay";
    nixfmt.url = "github:serokell/nixfmt";
  };

  outputs = { nixpkgs, systems, rust-overlay, nixfmt, ... }:
    let
      inherit (nixpkgs) lib;
      eachSystem = lib.genAttrs (import systems);
      pkgsFor = eachSystem (system:
        import nixpkgs {
          localSystem = system;
          overlays = [ rust-overlay.overlays.default ];
        });
    in {
      devShells = eachSystem (system:
        let pkgs = pkgsFor.${system};
        in {
          default = pkgs.mkShell {
            strictDeps = true;

            nativeBuildInputs = with pkgs; [
              rust-bin.stable.latest.default
              pkg-config
              openssl
            ];

            OPENSSL_LIB_DIR = "${lib.getLib pkgs.openssl}/lib";
            OPENSSL_DIR = lib.getDev pkgs.openssl;
            OPENSSL_NO_VENDOR = 1;

            # RUST_BACKTRACE = 1;
          };
        });

      formatter = eachSystem (system: nixfmt.packages.${system}.default);
    };
}
