use aes_gcm::{AeadCore, Aes256Gcm, Key, KeyInit, Nonce};
use aes_gcm::aead::{Aead, OsRng};
use aes_gcm::aead::rand_core::RngCore;
use rsa::pkcs1::EncodeRsaPublicKey;
use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};
use rsa::pkcs1::EncodeRsaPrivateKey;

#[test]
fn test_rsa_util() {
    let mut rng = rand::thread_rng();
    let bits = 4096;
    let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let pub_key = RsaPublicKey::from(&priv_key);
    // Encrypt
    let data = b"hello world";
    let enc_data = pub_key.encrypt(&mut rng, Pkcs1v15Encrypt, &data[..]).expect("failed to encrypt");
    assert_ne!(&data[..], &enc_data[..]);

    // Decrypt
    let dec_data = priv_key.decrypt(Pkcs1v15Encrypt, &enc_data).expect("failed to decrypt");
    assert_eq!(&data[..], &dec_data[..]);
    // 导出密钥
    let priv_pem = priv_key.write_pkcs1_pem_file(
        "private_key.pem",
        rsa::pkcs1::LineEnding::LF,
    ).expect("failed to export private key");
    let pub_pem = pub_key.write_pkcs1_pem_file(
        "public_key.pem",
        rsa::pkcs1::LineEnding::LF,
    ).expect("failed to export public key");
}

#[test]
fn test_aes() {
    let key = Aes256Gcm::generate_key(OsRng);
    let cipher = Aes256Gcm::new(&key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let ciphertext = cipher.encrypt(&nonce, b"plaintext message".as_ref()).unwrap();
    let plaintext = cipher.decrypt(&nonce, ciphertext.as_ref()).unwrap();
    assert_eq!(&plaintext, b"plaintext message");

}