use std::time::Instant;
use tfhe::prelude::*;
use tfhe::{generate_keys, set_server_key, ConfigBuilder, FheInt8};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();

    let config = ConfigBuilder::default().build();
    let (keys, server_keys) = generate_keys(config);
    set_server_key(server_keys);
    println!("Key generation & setup took: {:?}", start.elapsed());

    let clear_a: i8 = -121;
    let clear_b: i8 = 87;

    let enc_start = Instant::now();
    let a = FheInt8::try_encrypt(clear_a, &keys)?;
    let b = FheInt8::try_encrypt(clear_b, &keys)?;
    println!("Encryption took: {:?}", enc_start.elapsed());

    let cmp_start = Instant::now();
    let greater = a.gt(&b);
    let greater_or_equal = a.ge(&b);
    let lower = a.lt(&b);
    let lower_or_equal = a.le(&b);
    let equal = a.eq(&b);
    println!("Comparison operations took: {:?}", cmp_start.elapsed());

    let dec_start = Instant::now();
    let dec_gt = greater.decrypt(&keys);
    let dec_ge = greater_or_equal.decrypt(&keys);
    let dec_lt = lower.decrypt(&keys);
    let dec_le = lower_or_equal.decrypt(&keys);
    let dec_eq = equal.decrypt(&keys);
    println!("Decryption took: {:?}", dec_start.elapsed());

    let total_time = start.elapsed();
    println!("Total execution time: {:?}", total_time);

    assert_eq!(dec_gt, clear_a > clear_b);
    assert_eq!(dec_ge, clear_a >= clear_b);
    assert_eq!(dec_lt, clear_a < clear_b);
    assert_eq!(dec_le, clear_a <= clear_b);
    assert_eq!(dec_eq, clear_a == clear_b);

    Ok(())
}
