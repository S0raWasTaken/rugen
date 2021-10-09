# rugen
Simple CLI password generator made in RUST

## USAGE:
```bash
rugen -s <Password Size> -t <Password Type>
# TYPES: lowest, low, medium, high
```

## Build instructions:
> I will assume you already have RUST installed on your computer
```bash
git clone https://github.com/S0raWasTaken/rugen
cd rugen
cargo build --release
```
**OPTIONAL**: Installing the binary to a binary folder
```bash
sudo mv ./target/release/rugen /usr/local/bin
```

## TODO:
- Add `-n <Number of passwords>` to generate multiple passwords
- Write more todos
