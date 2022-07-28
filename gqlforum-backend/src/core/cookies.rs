/// Most are adapted from the `cookie` crate
use cookie::Cookie;
use hmac::{Hmac, Mac};
use sha2::Sha256;

// Keep these in sync, and keep the key len synced with the `signed` docs as
// well as the `KEYS_INFO` const in secure::Key
const BASE64_DIGEST_LEN: usize = 44;

pub fn sign_cookie_unchecked<'a>(mut cookie: Cookie<'a>, key: &[u8]) -> Cookie<'a> {
    let mut mac = Hmac::<Sha256>::new_from_slice(key).expect("good key");
    mac.update(cookie.value().as_bytes());
    let mut new_value = base64::encode(&mac.finalize().into_bytes());
    new_value.push_str(cookie.value());
    cookie.set_value(new_value);
    cookie
}

pub fn verify_cookie_unchecked<'a>(mut cookie: Cookie<'a>, key: &[u8]) -> Option<Cookie<'a>> {
    let value = verify_cookie_str_unchecked(cookie.value(), key)?;
    cookie.set_value(value);
    Some(cookie)
}

pub fn verify_cookie_str_unchecked(cookie_value: &str, key: &[u8]) -> Option<String> {
    // Missing or invalid digest
    if !cookie_value.is_char_boundary(BASE64_DIGEST_LEN) {
        return None;
    }
    // Split [MAC | original-value] into its two parts.
    let (digest_str, value) = cookie_value.split_at(BASE64_DIGEST_LEN);
    let digest = base64::decode(digest_str).ok()?;

    // Perform the verification.
    let mut mac = Hmac::<Sha256>::new_from_slice(key).expect("good key");
    mac.update(value.as_bytes());
    mac.verify_slice(&digest).map(|_| value.to_string()).ok()
}
