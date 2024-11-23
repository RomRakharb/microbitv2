{
  description = "Rust Embedded";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = { self, nixpkgs }:
  let
    pkgs = nixpkgs.legacyPackages.x86_64-linux;
  in
  {
    devShells.x86_64-linux.default = pkgs.mkShell {
      buildInputs = with pkgs; [
        rustup
        # gcc-arm-embedded
        # cargo-binutils
        # probe-rs-tools
        # minicom
      ];

      shellHook = ''
        rustup default stable
        rustup add component rust-analyzer
        rustup update
        clear
        # echo -e "# CMSIS-DAP for microbit\n\nACTION!=\"add|change\", GOTO=\"microbit_rules_end\"\n\nSUBSYSTEM==\"usb\", ATTR{idVendor}==\"0d28\", ATTR{idProduct}==\"0204\", TAG+=\"uaccess\"\n\nLABEL=\"microbit_rules_end\"" | sudo tee /etc/udev/rules.d/69-microbit.rules > /dev/null
        # sudo udevadm control --reload
        # sudo udevadm trigger
        # clear
        # echo "Development Environment for Embedded Rust"
        # echo "-------------------------------------------------"
        # rustc --version
        # echo "-------------------------------------------------"
        # lsusb | grep -i "NXP ARM mbed" | awk '{print "/dev/bus/usb/" $2 "/" $4}' | sed 's/://g' | xargs -I {} sh -c 'ls -l {}; echo "-------------------------------------------------"; getfacl {}'
        # echo "-------------------------------------------------"
        # probe-rs list
        # rustup target add thumbv7em-none-eabihf
        # cargo embed --target thumbv7em-none-eabihf
      '';
    };
  };
}
