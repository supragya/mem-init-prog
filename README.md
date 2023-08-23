# mem-init-prog
Memory initialization programs for testing

Example cargo project which can generate ELF compatible with MozakVM.

## Setup RISC-V toolchains on MacOSX
Using brew one can install RISC-V toolchain as given here: https://github.com/riscv-software-src/homebrew-riscv

## How to build: 
```
cargo build
```

## To dump assembly files: 
```
RUSTFLAGS="--emit asm" cargo build
```

Now check in ./target/riscv32im-risc0-zkvm-elf/debug/deps for assembly files (.s extension)