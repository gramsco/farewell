# Farewell

"It's dangerous to go alone! Take this."

A basic POC in rust 🦀 to download infos of any country referenced by [the french Ministry of Foreign Affairs](https://www.diplomatie.gouv.fr/fr/conseils-aux-voyageurs/) "Conseil aux voyageurs".

As of now (2024/01/03), this will only download the security map in downloads/{country_french_name}.jpg

## How to run the script

```rust
cargo run colombie
```

This will create a downloads/colombie.jpg file

## Robots.txt

The /robots directory contains the "diplomatie.gouv.fr" robots.txt as of 2024/01/03 as well as its shasum.

```bash
shasum -c robots/shasum
```
