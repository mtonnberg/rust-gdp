{
  description = "rust-gdp: checked evidence with erased proof plumbing";
  inputs.nixpkgs.url = "https://github.com/NixOS/nixpkgs/archive/fa1373130d5a21c040a47c5dbe1913d59ed91808.tar.gz";

  outputs = { self, nixpkgs }:
    let
      systems = [ "x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin" ];
      eachSystem = nixpkgs.lib.genAttrs systems;
      forSystem = system:
        let
          pkgs = import nixpkgs { inherit system; };
          tools = with pkgs; [ rustc cargo clippy rustfmt python3 ];
          crate = pkgs.rustPlatform.buildRustPackage {
            pname = "rust-gdp";
            version = "0.1.0";
            src = pkgs.lib.cleanSourceWith {
              src = self;
              filter = path: type:
                !(builtins.elem (baseNameOf path) [ ".git" "target" "result" ])
                && pkgs.lib.cleanSourceFilter path type;
            };
            cargoLock.lockFile = ./Cargo.lock;
            nativeBuildInputs = tools;
            doCheck = true;
            checkPhase = ''
              runHook preCheck
              export RUSTDOCFLAGS="-D warnings"
              bash scripts/verify.sh
              runHook postCheck
            '';
            installPhase = ''
              runHook preInstall
              mkdir -p $out/share/rust-gdp
              cp "''${CARGO_TARGET_DIR:-target}"/package/rust-gdp-*.crate $out/share/rust-gdp/
              runHook postInstall
            '';
            meta = {
              description = "Generative proofs and refinements for Rust";
              license = pkgs.lib.licenses.asl20;
              platforms = systems;
            };
          };
        in {
          packages.default = crate;
          checks.default = crate;
          devShells.default = pkgs.mkShell { packages = tools; };
        };
    in {
      packages = eachSystem (system: (forSystem system).packages);
      checks = eachSystem (system: (forSystem system).checks);
      devShells = eachSystem (system: (forSystem system).devShells);
    };
}
