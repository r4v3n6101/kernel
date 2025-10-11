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
        pkgs = import nixpkgs { inherit system overlays; };

        rustVersion = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain;

        naersk-lib = pkgs.callPackage naersk {
          cargo = rustVersion;
          rustc = rustVersion;
        };

        simpleboot = pkgs.stdenv.mkDerivation {
          name = "simpleboot";

          src = pkgs.fetchFromGitLab {
            owner = "bztsrc";
            repo = "simpleboot";
            rev = "b8c8a25c467e2a3043b4193bafba244ccd9074af";
            hash = "sha256-0sI6z6KBH67odkLTUFMw70OlPD4FR+eFWvldoupdrgw=";
          };

          buildPhase = ''
            cd src/
            make
          '';

          installPhase = ''
            mkdir -p $out/bin
            cp $name $out/bin/
          '';
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

        qemuTestKernel = pkgs.stdenv.mkDerivation {
          name = "qemuTestKernel";

          phases = [
            "buildPhase"
            "installPhase"
          ];

          buildInputs = [ pkgs.qemu ];
          nativeBuildInputs = [
            simpleboot
            kernel
            pkgs.makeWrapper
            pkgs.mktemp
          ];

          buildPhase = ''
            mkdir -p $out/share
            simpleboot ${kernel}/bin/ $out/share/kernel.img
          '';

          installPhase = ''
            mkdir -p $out/bin
            makeWrapper ${pkgs.qemu}/bin/qemu-system-x86_64 $out/bin/$name \
              --run "img=\$(mktemp /tmp/kernel.XXXXXX.img); cp $out/share/kernel.img \$img" \
              --add-flags "-m 8G -drive format=raw,file=\$img -serial stdio -display none -no-reboot -no-shutdown"
          '';
        };
      in
      {
        formatter = pkgs.nixpkgs-fmt;

        packages.default = qemuTestKernel;

        devShells.default = pkgs.mkShell {
          buildInputs = [
            (rustVersion.override (prev: {
              extensions = prev.extensions ++ [ "rust-analyzer" ];
            }))
          ];
        };
      }
    );
}
