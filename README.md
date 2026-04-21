# jkn_rust

Client Rust untuk bridging API BPJS/JKN.

## Status

Service yang sudah dipetakan ke crate ini:

- `vclaim`
- `pcare`
- `antrean`
- `antrean_fktp`
- `apotek`
- `icare`
- `rekam_medis`
- `aplicares`

Paritas yang sudah dikejar saat ini adalah surface endpoint dan perilaku request utama. Banyak request/response masih generik `serde_json::Value`, lalu bisa di-convert ke type Anda sendiri.

## Instalasi

```toml
[dependencies]
jkn_rust = { path = "../jkn_rust" }
serde = { version = "1", features = ["derive"] }
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

## Konfigurasi

Crate akan membaca `.env` melalui `dotenv`.

Lihat `.env.example`:

```env
NODE_ENV=development
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

## Catatan

- Method Rust mengikuti surface TypeScript, tetapi memakai naming snake_case.
- Payload request umumnya menerima `impl Serialize`, jadi Anda bisa memakai struct sendiri.
- Jika ingin type request/response bawaan per endpoint, itu masih bisa dilanjutkan sebagai tahap berikutnya.
