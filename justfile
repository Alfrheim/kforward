default:
    cargo run

run:
    cargo run 

release:
    cargo build --release

copy-bin:
    cp --verbose --recursive target/release/kforward ~/bin/
