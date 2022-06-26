{ pkgs, pre-commit-hooks, system }:

with pkgs;

let
  pre-commit-check = pre-commit-hooks.lib.${system}.run {
    src = ./.;
    hooks = {
      cargo-check.enable = true;
      nixpkgs-fmt.enable = true;
      nix-linter.enable = true;
      rustfmt.enable = true;
      shellcheck.enable = true;
      statix.enable = true;
    };
    # generated files
    excludes = [
    ];
  };

  rust = rust-bin.nightly."2022-06-10".default.override {
    extensions = [ "rust-src" "rust-analyzer-preview" ];
  };
in
mkShell {
  buildInputs = [
    cacert
    cargo-edit
    cargo-outdated
    cmake
    openssl
    pkgconfig
    rust
    rustfmt
  ];

  RUST_SRC_PATH = "${rust}/lib/rustlib/src/rust/library";

  shellHook = ''
    ${pre-commit-check.shellHook}
  '';
}
