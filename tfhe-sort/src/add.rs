use tfhe::prelude::*;
// use tfhe::{generate_keys, set_server_key, ConfigBuilder, FheUint8}; // For addition
use tfhe::{generate_keys, set_server_key, ConfigBuilder, FheInt8, FheUint8}; // For comparators

fn add() {
    println!("Initializing TFHE configuration...");
    let config = ConfigBuilder::default().build();

    println!("Generating client and server keys...");
    let (client_key, server_key) = generate_keys(config);

    // Clear values before encryption
    let clear_a = 27u8;
    let clear_b = 128u8;
    println!(
        "Original values: clear_a = {} , clear_b = {}",
        clear_a, clear_b
    );

    // Encrypt values (Client-side encryption)
    println!("Encrypting values...");
    let a = FheUint8::encrypt(clear_a, &client_key);
    let b = FheUint8::encrypt(clear_b, &client_key);
    println!("Encryption complete.");

    // Server-side computation
    println!("Setting server key...");
    set_server_key(server_key);
    println!("Performing encrypted addition...");
    let result = &a + &b;
    println!("Addition complete.");

    // Decrypt result (Client-side decryption)
    println!("Decrypting result...");
    let decrypted_result: u8 = result.decrypt(&client_key);
    println!("Decryption complete. Result = {}", decrypted_result);

    // Verify correctness
    let clear_result = clear_a + clear_b;
    println!("Expected result (unencrypted) = {}", clear_result);

    assert_eq!(decrypted_result, clear_result);
    println!("Assertion passed! Encrypted computation is correct.");
}

fn compare() -> Result<(), Box<dyn std::error::Error>> {
    let config = ConfigBuilder::default().build();
    let (keys, server_keys) = generate_keys(config);
    set_server_key(server_keys);

    let clear_a: i8 = -121;
    let clear_b: i8 = 87;

    let a = FheInt8::try_encrypt(clear_a, &keys)?;
    let b = FheInt8::try_encrypt(clear_b, &keys)?;

    let greater = a.gt(&b);
    let greater_or_equal = a.ge(&b);
    let lower = a.lt(&b);
    let lower_or_equal = a.le(&b);
    let equal = a.eq(&b);

    let dec_gt = greater.decrypt(&keys);
    let dec_ge = greater_or_equal.decrypt(&keys);
    let dec_lt = lower.decrypt(&keys);
    let dec_le = lower_or_equal.decrypt(&keys);
    let dec_eq = equal.decrypt(&keys);

    assert_eq!(dec_gt, clear_a > clear_b);
    assert_eq!(dec_ge, clear_a >= clear_b);
    assert_eq!(dec_lt, clear_a < clear_b);
    assert_eq!(dec_le, clear_a <= clear_b);
    assert_eq!(dec_eq, clear_a == clear_b);

    Ok(())
}

fn main() {
    // add();
    compare().unwrap();
}
/////////////////////////
