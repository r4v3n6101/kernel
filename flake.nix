{
  description = "Another kernel project for learning purpose";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";

    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      naersk,
      rust-overlay,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ rust-overlay.overlays.default ];
        pkgs = import nixpkgs {
          inherit system overlays;

          config.allowBroken = true;
        };

        rustVersion = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain;

        naersk-lib = pkgs.callPackage naersk {
          cargo = rustVersion;
          rustc = rustVersion;
        };

        kernel = naersk-lib.buildPackage (
          let
            manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
          in
          {
            inherit (manifest) name version;

            root = ./.;

            # ./targets/* won't seen otherwise
            singleStep = true;
            usePureFromTOML = false;
            additionalCargoLock = "${rustVersion}/lib/rustlib/src/rust/library/Cargo.lock";
          }
        );

        qemu-vm =
          kernelPath:
          pkgs.stdenv.mkDerivation {
            name = "qemuTestKernel";

            phases = [
              "installPhase"
            ];

            buildInputs = [ pkgs.qemu ];
            nativeBuildInputs = [ pkgs.makeWrapper ];

            installPhase = ''
              mkdir -p $out/bin
              makeWrapper ${pkgs.qemu_full}/bin/qemu-system-aarch64 $out/bin/$name --add-flags " \
                  -S -s -nographic \
                  -M raspi4b \
                  -kernel ${kernelPath} \
                  -dtb ${pkgs.device-tree_rpi}/broadcom/bcm2711-rpi-4-b.dtb \
              "
            '';
          };
      in
      {
        formatter = pkgs.nixpkgs-fmt;

        packages.default = qemu-vm "${kernel}/bin/kernel";
        devShells.default = pkgs.mkShell {
          buildInputs = [
            pkgs.gdb
            (qemu-vm "target/aarch64-unknown-none/debug/kernel")
            (rustVersion.override (prev: {
              extensions = prev.extensions ++ [ "rust-analyzer" ];
            }))
          ];

          LD_SCRIPT = "rpi4.ld";
        };
      }
    );
}
