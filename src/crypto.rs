use std::collections::HashMap;
use std::io::Write;

use base64::Engine;
use flate2::Compression;
use flate2::write::GzEncoder;
use openssl::hash::{MessageDigest, hash};
use openssl::pkey::PKey;
use openssl::sign::Signer;
use openssl::symm::{Cipher, Crypter, Mode};

use crate::error::{JknError, Result};

pub fn hmac_sha256_base64(secret: &[u8], message: &[u8]) -> Result<String> {
    let key = PKey::hmac(secret).map_err(|err| JknError::Crypto(err.to_string()))?;
    let mut signer = Signer::new(MessageDigest::sha256(), &key)
        .map_err(|err| JknError::Crypto(err.to_string()))?;
    signer
        .update(message)
        .map_err(|err| JknError::Crypto(err.to_string()))?;
    let signature = signer
        .sign_to_vec()
        .map_err(|err| JknError::Crypto(err.to_string()))?;
    Ok(base64::engine::general_purpose::STANDARD.encode(signature))
}

pub fn sha256_bytes(input: &[u8]) -> Result<Vec<u8>> {
    hash(MessageDigest::sha256(), input)
        .map(|digest| digest.to_vec())
        .map_err(|err| JknError::Crypto(err.to_string()))
}

pub fn aes_256_cbc_decrypt_base64(input: &str, plain_key: &str) -> Result<String> {
    let key = sha256_bytes(plain_key.as_bytes())?;
    let iv = &key[..16];
    let decoded = base64::engine::general_purpose::STANDARD
        .decode(input)
        .map_err(|err| JknError::Crypto(err.to_string()))?;
    let cipher = Cipher::aes_256_cbc();
    let mut crypter = Crypter::new(cipher, Mode::Decrypt, &key, Some(iv))
        .map_err(|err| JknError::Crypto(err.to_string()))?;
    crypter.pad(true);

    let mut out = vec![0_u8; decoded.len() + cipher.block_size()];
    let mut count = crypter
        .update(&decoded, &mut out)
        .map_err(|err| JknError::Crypto(err.to_string()))?;
    count += crypter
        .finalize(&mut out[count..])
        .map_err(|err| JknError::Crypto(err.to_string()))?;
    out.truncate(count);

    String::from_utf8(out).map_err(|err| JknError::Crypto(err.to_string()))
}

pub fn aes_256_cbc_encrypt_base64(input: &[u8], plain_key: &str) -> Result<String> {
    let key = sha256_bytes(plain_key.as_bytes())?;
    let iv = &key[..16];
    let cipher = Cipher::aes_256_cbc();
    let mut crypter = Crypter::new(cipher, Mode::Encrypt, &key, Some(iv))
        .map_err(|err| JknError::Crypto(err.to_string()))?;
    crypter.pad(true);

    let mut out = vec![0_u8; input.len() + cipher.block_size()];
    let mut count = crypter
        .update(input, &mut out)
        .map_err(|err| JknError::Crypto(err.to_string()))?;
    count += crypter
        .finalize(&mut out[count..])
        .map_err(|err| JknError::Crypto(err.to_string()))?;
    out.truncate(count);

    Ok(base64::engine::general_purpose::STANDARD.encode(out))
}

pub fn gzip_base64(input: &str) -> Result<String> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(input.as_bytes())?;
    let bytes = encoder.finish()?;
    Ok(base64::engine::general_purpose::STANDARD.encode(bytes))
}

pub fn decrypt_response_payload(
    encrypted: &str,
    cons_id: &str,
    cons_secret: &str,
    timestamp: &str,
) -> Result<String> {
    let plain_key = format!("{cons_id}{cons_secret}{timestamp}");
    let decrypted = aes_256_cbc_decrypt_base64(encrypted, &plain_key)?;
    lzstring_decompress_from_encoded_uri_component(&decrypted)
}

pub fn encrypt_rekam_medis_payload(
    plain_json: &str,
    cons_id: &str,
    cons_secret: &str,
    ppk_code: &str,
) -> Result<String> {
    let compressed = gzip_base64(plain_json)?;
    let plain_key = format!("{cons_id}{cons_secret}{ppk_code}");
    aes_256_cbc_encrypt_base64(compressed.as_bytes(), &plain_key)
}

pub fn parse_html(html: &str) -> String {
    let mut result = html.to_string();
    for pattern in [
        r"(?is)<head\b[^>]*>.*?</head>",
        r"(?is)<script.*?</script>",
        r"(?is)<style.*?</style>",
    ] {
        let re = regex_like_replace(pattern, &result);
        result = re;
    }
    let stripped = strip_tags(&result);
    stripped
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .replace('\n', " - ")
}

fn strip_tags(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut in_tag = false;
    for ch in input.chars() {
        match ch {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => out.push(ch),
            _ => {}
        }
    }
    out.trim().to_string()
}

fn regex_like_replace(pattern: &str, input: &str) -> String {
    let (start_tag, end_tag) = match pattern {
        r"(?is)<head\b[^>]*>.*?</head>" => ("<head", "</head>"),
        r"(?is)<script.*?</script>" => ("<script", "</script>"),
        r"(?is)<style.*?</style>" => ("<style", "</style>"),
        _ => return input.to_string(),
    };

    let mut result = String::new();
    let mut cursor = 0;
    let lower = input.to_lowercase();
    while let Some(start) = lower[cursor..].find(start_tag) {
        let start = cursor + start;
        result.push_str(&input[cursor..start]);
        let Some(end) = lower[start..].find(end_tag) else {
            cursor = input.len();
            break;
        };
        cursor = start + end + end_tag.len();
    }
    result.push_str(&input[cursor..]);
    result
}

const KEY_STR_URI_SAFE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+-$";

pub fn lzstring_decompress_from_encoded_uri_component(input: &str) -> Result<String> {
    if input.is_empty() {
        return Ok(String::new());
    }

    let normalized = input.replace(' ', "+");
    let alphabet: HashMap<char, usize> = KEY_STR_URI_SAFE
        .chars()
        .enumerate()
        .map(|(i, ch)| (ch, i))
        .collect();

    let chars: Vec<char> = normalized.chars().collect();
    let mut data = DecompressData {
        val: *alphabet
            .get(
                chars
                    .first()
                    .ok_or_else(|| JknError::Crypto("empty payload".to_string()))?,
            )
            .ok_or_else(|| JknError::Crypto("invalid lz-string alphabet".to_string()))?
            as u32,
        position: 32,
        index: 1,
        chars,
        alphabet,
    };

    lz_decompress(&mut data)
}

struct DecompressData {
    val: u32,
    position: u32,
    index: usize,
    chars: Vec<char>,
    alphabet: HashMap<char, usize>,
}

impl DecompressData {
    fn next_bits(&mut self, count: u32) -> Result<u32> {
        let mut power = 1_u32;
        let mut bits = 0_u32;

        while power != (1 << count) {
            let resb = self.val & self.position;
            self.position >>= 1;
            if self.position == 0 {
                self.position = 32;
                let ch = self.chars.get(self.index).ok_or_else(|| {
                    JknError::Crypto("unexpected end of compressed payload".to_string())
                })?;
                self.index += 1;
                self.val = *self
                    .alphabet
                    .get(ch)
                    .ok_or_else(|| JknError::Crypto("invalid lz-string alphabet".to_string()))?
                    as u32;
            }

            if resb > 0 {
                bits |= power;
            }
            power <<= 1;
        }

        Ok(bits)
    }
}

fn lz_decompress(data: &mut DecompressData) -> Result<String> {
    let mut dictionary: Vec<String> = vec![String::new(), String::new(), String::new()];
    let mut enlarge_in = 4_u32;
    let mut dict_size = 4_u32;
    let mut num_bits = 3_u32;

    let next = data.next_bits(2)?;
    let c = match next {
        0 => std::char::from_u32(data.next_bits(8)?)
            .ok_or_else(|| JknError::Crypto("invalid utf-8 code point".to_string()))?
            .to_string(),
        1 => std::char::from_u32(data.next_bits(16)?)
            .ok_or_else(|| JknError::Crypto("invalid utf-16 code point".to_string()))?
            .to_string(),
        2 => return Ok(String::new()),
        _ => return Err(JknError::Crypto("invalid lz-string header".to_string())),
    };

    dictionary.push(c.clone());
    let mut w = c.clone();
    let mut result = c;

    loop {
        let mut cc = data.next_bits(num_bits)?;
        if cc == 0 {
            let ch = std::char::from_u32(data.next_bits(8)?)
                .ok_or_else(|| JknError::Crypto("invalid utf-8 code point".to_string()))?;
            dictionary.push(ch.to_string());
            cc = dict_size;
            dict_size += 1;
            enlarge_in = enlarge_in.saturating_sub(1);
        } else if cc == 1 {
            let ch = std::char::from_u32(data.next_bits(16)?)
                .ok_or_else(|| JknError::Crypto("invalid utf-16 code point".to_string()))?;
            dictionary.push(ch.to_string());
            cc = dict_size;
            dict_size += 1;
            enlarge_in = enlarge_in.saturating_sub(1);
        } else if cc == 2 {
            return Ok(result);
        }

        if enlarge_in == 0 {
            enlarge_in = 1 << num_bits;
            num_bits += 1;
        }

        let entry = if let Some(value) = dictionary.get(cc as usize) {
            value.clone()
        } else if cc == dict_size {
            let first = w
                .chars()
                .next()
                .ok_or_else(|| JknError::Crypto("invalid lz-string state".to_string()))?;
            format!("{w}{first}")
        } else {
            return Err(JknError::Crypto(
                "invalid lz-string dictionary index".to_string(),
            ));
        };

        result.push_str(&entry);
        let first = entry
            .chars()
            .next()
            .ok_or_else(|| JknError::Crypto("invalid lz-string entry".to_string()))?;
        dictionary.push(format!("{w}{first}"));
        dict_size += 1;
        enlarge_in = enlarge_in.saturating_sub(1);
        w = entry;

        if enlarge_in == 0 {
            enlarge_in = 1 << num_bits;
            num_bits += 1;
        }
    }
}
