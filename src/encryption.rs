pub mod my_enc {
    use aes_gcm::aead::{Aead, NewAead};
    use aes_gcm::{Aes256Gcm, Key, Nonce};
    use rand_chacha::ChaCha20Rng;
    use rand_chacha::rand_core::SeedableRng;
    use rand_core::RngCore;
    use rand_chacha;
    use std::convert::TryInto;
    use std::fs;

    fn random_nonce_key() -> [u8; 12]{
        //TODO: make random seed 
        let mut rng = ChaCha20Rng::seed_from_u64(59);
        let result: Vec<u8> = (0..12).map(|_| rng.next_u64() as u8).collect();
        result.try_into().expect("Error: Counld not generate nonce key!")
    }

    pub fn encrypt_file(file_to_encrypt: &str, target: &str) {
        let line = fs::read(file_to_encrypt).expect("Could read from file_to_encrypt.");
        let my_pass = b"an example very very secret key.";
        let ciphertext = encryption_step(&line[..], &my_pass[..]);
        fs::write(target, ciphertext).expect("Could not write to file_to_encrypt.");
    }

    pub fn decrypt_file(file_to_decrypt: &str, target: &str) {
        let line = fs::read(file_to_decrypt).unwrap();
        let my_pass = b"an example very very secret key.";
        let plaintext = decryption_step(&line[..], &my_pass[..]);
        fs::write(target, plaintext).unwrap();
    }

    fn encryption_step(plaintext: &[u8], password: &[u8]) -> Vec<u8> {
        let cipher = Aes256Gcm::new(Key::from_slice(password));
        let nonce = random_nonce_key();
        let nonce = Nonce::from_slice(&nonce);
        cipher
            .encrypt(nonce, plaintext)
            .expect("encryption failure!")
    }

    fn decryption_step(ciphertext: &[u8], password: &[u8]) -> Vec<u8> {
        let cipher = Aes256Gcm::new(Key::from_slice(password));
        let nonce = random_nonce_key();
        let nonce = Nonce::from_slice(&nonce);
        cipher
            .decrypt(nonce, ciphertext)
            .expect("encryption failure!")
    }
}
