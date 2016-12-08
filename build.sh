#!/usr/bin/sh
cargo build --release --example opt-f32 \
    && rustc examples/opt-f32.rs --crate-name opt_f32 --crate-type bin -C opt-level=3 -C metadata=0119d20f27a7d5be --out-dir target/release/examples --emit=asm,llvm-ir -L dependency=target/release/deps --extern llvm_opt=target/release/deps/libllvm_opt.rlib
cargo build --release --example opt-v32 \
    && rustc examples/opt-v32.rs --crate-name opt_v32 --crate-type bin -C opt-level=3 -C metadata=0119d20f27a7d5be --out-dir target/release/examples --emit=asm,llvm-ir -L dependency=target/release/deps --extern llvm_opt=target/release/deps/libllvm_opt.rlib
cargo build --release --example opt-p32 \
    && rustc examples/opt-p32.rs --crate-name opt_p32 --crate-type bin -C opt-level=3 -C metadata=0119d20f27a7d5be --out-dir target/release/examples --emit=asm,llvm-ir -L dependency=target/release/deps --extern llvm_opt=target/release/deps/libllvm_opt.rlib

rustup run nightly cargo bench --bench bench
