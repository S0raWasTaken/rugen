# Rugen
Simple CLI password generator made in RUST

## USAGE:
```bash
# TYPES: lowest, low, medium, high
rugen -s <Password Size> -t <Password Type>
```

## Build instructions:
> I will assume you already have RUST installed in your system
```bash
git clone https://github.com/S0raWasTaken/rugen
cd rugen
cargo build --release
```
**OPTIONAL**: Installing the binary to a bin folder
```bash
sudo mv ./target/release/rugen /usr/local/bin
```

## TODO:
- Add `-n <Number of passwords>` to generate multiple passwords
- Write more todos
