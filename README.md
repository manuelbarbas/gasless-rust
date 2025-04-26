# Gasless-Rust

This project is designed to run SKALE Chains gasless transactions in Rust due to the language great performance.

## Rust Install

<details>
<summary>Windows</summary>

1. Visit the official Rust website at https://www.rust-lang.org/tools/install
2. The recommended way to install Rust is using "rustup," the Rust installer and version management tool.


### Verify installation:
rustc --version
cargo --version
</details>

<details>
<summary>MacOS</summary>

**Run:**

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Verify installation:
rustc --version
cargo --version

</details>

<details>
<summary>Linux</summary>

**Run:**
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Verify installation:
rustc --version
cargo --version
</details>

## Repo setup

1. Clone the Repo

2. Head to `skale-gasless` folder by running:
```sh
cd skale-gasless/
``` 

3. Compile the repo to import the project dependencies: 
```sh
cargo build --release
```

## Run a gasless transaction

The current setup is running a gasless transaction which calls the sFUEL faucet contract to perform sFUEL Distribution:
```sh
cargo run
```