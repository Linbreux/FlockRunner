{
  description = "FlockRunner";

  inputs =
    {
      nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    };

  outputs = { self, nixpkgs, ... }@inputs:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};

      rustPlatform = pkgs.rustPlatform;
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        packages = with pkgs; [
          rustc
          cargo
          rust-analyzer
        ];
      };
      packages.${system}.flockrunner = rustPlatform.buildRustPackage {
        pname = "flockrunner"; # Name of your application
        version = "0.1.0"; # Version of your application

        # The source code for the package is the current directory
        src = self;

        # Dependencies needed for building the project (e.g., pkg-config for C libraries)
        nativeBuildInputs = with pkgs; [
          pkg-config # Common for Rust projects that link against C libraries
        ];

        # Runtime dependencies (e.g., OpenSSL, SQLite)
        buildInputs = with pkgs; [
          # openssl.dev
          # sqlite.dev
        ];

        cargoLock = {
          lockFile = ./Cargo.lock;
        };

      };

      defaultPackage.${system} = self.packages.${system}.flockrunner;

      apps.${system}.default = {
        type = "app";
        program = "${self.packages.${system}.flockrunner}/bin/fr";
      };
    };
}

