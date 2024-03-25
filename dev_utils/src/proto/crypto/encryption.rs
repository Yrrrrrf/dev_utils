//! Encryption Module
//! 
//! This module contains the encryption functions for the project.
//! 
//! This module impl the 3 most common encryption algorithms:
//! - AES (256) - Advanced Encryption Standard (Symmetric)
//! - RSA - Rivest–Shamir–Adleman (Asymmetric)
//! - Diffie-Hellman (Key Exchange) (Asymmetri


fn main() {
    // Step 1: Generate two large prime numbers, p and q
    // let p = generate_prime(512);
    // let q = generate_prime(512);
    let p: u32 = 13;
    let q: u32 = 7;

    // Step 2: Compute n = p * q
    let n = &p * &q;  // n is part of the public key

    // Step 3: Compute φ(n) = (p - 1) * (q - 1)
    let phi_n = (&p - 1) * (&q - 1);  // phi_n is part of the private key

    // Step 4: Choose an encryption key e such that 1 < e < φ(n) and gcd(e, φ(n)) = 1
    // let e = generate_public_exponent(&phi_n);

    // // Step 5: Compute the private key d such that (d * e) % φ(n) == 1
    // let d = modular_inverse(&e, &phi_n);

    // // Step 6: Message to be encrypted
    // // let plaintext = BigUint::from(42u64);

    // // Step 7: Encrypt the message
    // let ciphertext = encrypt(&plaintext, &e, &n);

    // // Step 8: Decrypt the ciphertext
    // let decrypted_message = decrypt(&ciphertext, &d, &n);

    // // Display the results
    // println!("Original Message: {}", plaintext);
    // println!("Encrypted Message: {}", ciphertext);
    // println!("Decrypted Message: {}", decrypted_message);
}


