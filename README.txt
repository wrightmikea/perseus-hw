to recreate Rust nightly compiler issue similar to this one?
> https://github.com/rust-lang/rust/issues/104876

1. rustup default nightly
2. rustup update
3. cargo install perseus-cli --version 0.4.0-beta.11
4. clone and cd to this repo (https://github.com/wrightmikea/perseus-hw)
5. perseus serve
...
compare output to ./error.txt 

