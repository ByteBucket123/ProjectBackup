//use ring::{aead, hkdf};
use aes_gcm::aead::{Aead, NewAead};
use aes_gcm::{Aes256Gcm, Key, Nonce};
use std::str;

mod encryption;

pub use crate::encryption::my_enc;

fn main() {
    println!("Hello, world!");
    my_enc::encrypt_file(
        ".\\src\\file_to_encrypt.txt", 
        ".\\src\\src\\file_to_decrypt.txt"
    );
    my_enc::decrypt_file(
        ".\\src\\src\\file_to_decrypt.txt",
        ".\\src\\src\\decrypted_file.txt"
    );
    //test_encrypt();
}

fn test_encrypt() {
    let key = Key::from_slice(b"an example very very secret key.");
    let cipher = Aes256Gcm::new(key);

    let nonce = Nonce::from_slice(b"unique nonce"); // 96-bits; unique per message

    let plaintext = b"plaintext message";

    let ciphertext = cipher
        .encrypt(nonce, plaintext.as_ref())
        .expect("encryption failure!"); // NOTE: handle this error to avoid panics!

    println!("Ciphertext: {:?}", String::from_utf8_lossy(&ciphertext[..]));

    let plaintext = cipher
        .decrypt(nonce, ciphertext.as_ref())
        .expect("decryption failure!"); // NOTE: handle this error to avoid panics!

    println!("Plaintext: {:?}", str::from_utf8(&plaintext[..]).unwrap());
}

/* ring test
fn test_encrypt(){
    let key = b"password";
    let unbound_key: aead::UnboundKey = hkdf::Prk::new_less_safe(hkdf::HKDF_SHA256, key)
        .expand(&[], &aead::AES_256_GCM).unwrap().into();
    let less_safe_sealing_key = aead::LessSafeKey::new(unbound_key);

    let mut nonce_buf = [0u8; aead::NONCE_LEN];
    let nonce = &mut nonce_buf[..aead::NONCE_LEN];
    let nonce = aead::Nonce::try_assume_unique_for_key(nonce).unwrap();

    let mut buf = [1, 2, 3];

    let tag = less_safe_sealing_key.seal_in_place_separate_tag(nonce, aead::Aad::empty(), &mut buf).unwrap();

    let unbound_key: aead::UnboundKey = hkdf::Prk::new_less_safe(hkdf::HKDF_SHA256, key)
        .expand(&[], &aead::AES_256_GCM).unwrap().into();
    let less_safe_sealing_key = aead::LessSafeKey::new(unbound_key);

    let mut nonce_buf = [0u8; aead::NONCE_LEN];
    let nonce = &mut nonce_buf[..aead::NONCE_LEN];
    let nonce = aead::Nonce::try_assume_unique_for_key(nonce).unwrap();

    let result = less_safe_sealing_key.open_in_place(nonce, aead::Aad::empty(), &mut buf).unwrap();

    println!("Result: {}, {}, {}", result[0], result[1], result[2]);
    //let bound_key = aead::BoundKey::new(unbound_key, nonce_sequence);
}
fn encrypt(&self, packet: u64, buf: &mut [u8], header_len: usize) {
    let (cipher, iv, key) = (
        self.sealing_key.algorithm(),
        &self.local_iv,
        &self.sealing_key,
    );

    let mut nonce_buf = [0u8; aead::NONCE_LEN];
    let nonce = &mut nonce_buf[..cipher.nonce_len()];
    self.write_nonce(iv, packet, nonce);

    let (header, payload) = buf.split_at_mut(header_len);
    let (payload, tag) = payload.split_at_mut(payload.len() - cipher.tag_len());
    let header = aead::Aad::from(header);
    let nonce = aead::Nonce::try_assume_unique_for_key(nonce).unwrap();
    let tagged = key
        .seal_in_place_separate_tag(nonce, header, payload)
        .unwrap();

    tag.copy_from_slice(tagged.as_ref());
}

fn decrypt(&self, packet: u64, header: &[u8], payload: &mut BytesMut) -> Result<(), ()> {
    if payload.len() < self.tag_len() {
        return Err(());
    }

    let (cipher, iv, key) = (
        self.opening_key.algorithm(),
        &self.remote_iv,
        &self.opening_key,
    );

    let mut nonce_buf = [0u8; aead::NONCE_LEN];
    let nonce = &mut nonce_buf[..cipher.nonce_len()];
    self.write_nonce(iv, packet, nonce);
    let payload_len = payload.len();

    let header = aead::Aad::from(header);
    let nonce = aead::Nonce::try_assume_unique_for_key(nonce).unwrap();
    key.open_in_place(nonce, header, payload.as_mut())
        .map_err(|_| ())?;
    payload.split_off(payload_len - cipher.tag_len());
    Ok(())
}
*/
