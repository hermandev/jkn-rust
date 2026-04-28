[![Crates.io](https://img.shields.io/crates/v/jkn_rust)](https://crates.io/crates/jkn_rust)
[![Documentation](https://docs.rs/jkn_rust/badge.svg)](https://docs.rs/jkn_rust)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

# JKN Client Rust untuk bridging API BPJS/JKN


## Status Service

- `vclaim`
- `pcare`
- `antrean`
- `antrean_fktp`
- `apotek`
- `icare`
- `rekam_medis`
- `aplicares`


## Instalasi

```toml
[dependencies]
jkn_rust = "*"
serde = { version = "1", features = ["derive"] }
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

## Konfigurasi

Crate akan membaca `.env` melalui `dotenv`.

Lihat `.env.example`:

```env
JKN_MODE=development
JKN_PPK_CODE=
JKN_CONS_ID=
JKN_CONS_SECRET=
JKN_VCLAIM_USER_KEY=
JKN_ANTREAN_USER_KEY=
JKN_APOTEK_USER_KEY=
JKN_PCARE_USER_KEY=
JKN_PCARE_USER_PASS=
JKN_ICARE_USER_KEY=
JKN_REKAM_MEDIS_USER_KEY=
JKN_APLICARES_USER_KEY=
```

## Penggunaan

```rust
use jkn_rust::Jkn;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let jkn = Jkn::from_env()?;

    let result = jkn
        .vclaim()
        .referensi()
        .faskes("silampari", 2)
        .await?;

    println!("success: {}", result.is_success());
    println!("{:#?}", result.response);

    Ok(())
}
```

## Typed Response

Kalau ingin response yang typed, definisikan struct sendiri lalu parse dari `response`.

```rust
use jkn_rust::Jkn;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct FaskesItem {
    kode: String,
    nama: String,
}

#[derive(Debug, Deserialize)]
struct FaskesPayload {
    faskes: Vec<FaskesItem>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let jkn = Jkn::from_env()?;
    let result = jkn
        .vclaim()
        .referensi()
        .faskes("silampari", 2)
        .await?;

    let payload: FaskesPayload = result.response_as()?;
    println!("{:#?}", payload.faskes);

    Ok(())
}
```


