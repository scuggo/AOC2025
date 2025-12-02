{
  description = "";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    crane.url = "github:ipetkov/crane";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    {
      nixpkgs,
      crane,
      flake-utils,
      rust-overlay,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };
        craneLib = (crane.mkLib pkgs).overrideToolchain (
          p:
          p.rust-bin.nightly.latest.default.override {
            extensions = [
              "rustc-codegen-cranelift-preview"
              "rust-analyzer"
              "rust-src"
            ];
          }
        );
      in
      {
        devShells.default = craneLib.devShell {
          packages = [ ];
        };
      }
    );
}
