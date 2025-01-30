use std::time::Instant;

fn main() {
    let start = Instant::now();

    let clear_a: i8 = -121;
    let clear_b: i8 = 87;

    println!("Key generation & setup took: {:?}", start.elapsed());

    let enc_start = Instant::now();
    let a = clear_a;
    let b = clear_b;
    println!(
        "Encryption (plain variable assignment) took: {:?}",
        enc_start.elapsed()
    );

    let cmp_start = Instant::now();
    let greater = a > b;
    let greater_or_equal = a >= b;
    let lower = a < b;
    let lower_or_equal = a <= b;
    let equal = a == b;
    println!("Comparison operations took: {:?}", cmp_start.elapsed());

    let dec_start = Instant::now();
    let dec_gt = greater;
    let dec_ge = greater_or_equal;
    let dec_lt = lower;
    let dec_le = lower_or_equal;
    let dec_eq = equal;
    println!(
        "Decryption (direct assignment) took: {:?}",
        dec_start.elapsed()
    );

    let total_time = start.elapsed();
    println!("Total execution time: {:?}", total_time);

    assert_eq!(dec_gt, clear_a > clear_b);
    assert_eq!(dec_ge, clear_a >= clear_b);
    assert_eq!(dec_lt, clear_a < clear_b);
    assert_eq!(dec_le, clear_a <= clear_b);
    assert_eq!(dec_eq, clear_a == clear_b);
}
