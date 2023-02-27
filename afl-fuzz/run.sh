cargo afl build --workspace
cargo afl fuzz -i afl-fuzz/in-fuzz1 -o afl-fuzz/out-fuzz1 target/debug/afl-fuzz

#RUSTFLAGS=-Zsanitizer=address cargo afl build -Zbuild-std --target x86_64-unknown-linux-gnu --workspace
#RUSTFLAGS=-Zsanitizer=address cargo afl fuzz -i afl-fuzz/in-fuzz1 -o afl-fuzz/out-fuzz1 -m none target/x86_64-unknown-linux-gnu/debug/afl-fuzz