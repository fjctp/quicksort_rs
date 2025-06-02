clean: 
  cargo clean

build: clean
  cargo build

test: clean
  # Show `print` output.
  cargo test -- --nocapture

bench:
  cargo bench
