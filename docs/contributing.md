# Contributing

## Linux Mint

1. Install rustup [here](https://www.rust-lang.org/tools/install).
2. Install llvm 12.0.0 [here](https://github.com/llvm/llvm-project/releases/tag/llvmorg-12.0.0).
    - I extracted to `~/llvm/` and renamed the containing folder to `12.0.0`. The executables were then at `~/llvm/12.0.0/bin`.
3. Add to your `.bashrc`
    - `export PATH="$HOME/llvm/12.0.0/bin"` 
    - `export LLVM_SYS_120_PREFIX="$HOME/llvm/12.0.0"`
4. Compile project at root of disc folder with `cargo build`
