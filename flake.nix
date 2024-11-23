{
  description = "Rust Embedded";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = { self, nixpkgs }:
  let
    pkgs = nixpkgs.legacyPackages.x86_64-linux;
    target = "thumbv7em-none-eabihf";
  in
  {
    devShells.x86_64-linux.default = pkgs.mkShell {
      buildInputs = with pkgs; [
        rustup
        cargo-binutils
        probe-rs-tools
        gcc-arm-embedded-13
      ];


      shellHook = ''
        rustup default stable
        rustup component add rust-analyzer llvm-tools
        rustup target add ${target}
        echo -e '[language-server.rust-analyzer.config]\ncheck.command = "clippy"\ncheck.allTargets = false\ncargo.target = "${target}"' > ~/.config/helix/languages.toml
        echo -e '[build]\ntarget = "${target}"\n\n[target.${target}]\nrustflags = ["-C", "link-arg=-Tlink.x"]' > ./.cargo/config.toml
        rustup update
        clear
        echo "Development Environment for Embedded Rust"
        rustup show
      '';
    };
  };
}
