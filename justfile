default:
  @echo "Call just --list to see available tasks"
build-x86-apple:
  cargo build --release --target x86_64-apple-darwin
build-arm-apple:
  cargo build --release --target aarch64-apple-darwin
