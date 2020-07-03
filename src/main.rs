use scrypt::{ScryptParams, scrypt_simple};
use std::time::{Instant, Duration};

const N: usize = 100;

fn main() {
    println!("Starting smoketest");
    let mut hashed = "".to_owned();

    for i in 0.. {
        let params = ScryptParams::new(i, 8, 1).unwrap();
        let mut total = 0;

        for j in 0..N {
            let password = format!("Not so secure password {}", j);
            let start = Instant::now();
            hashed = scrypt_simple(&password, &params)
                .unwrap();
            let end = Instant::now();
            let time: Duration = end - start;
            total += time.as_nanos();
        }

        let ms_hash = (total / N as u128) as f64 / 1_000_000 as f64;

        println!(
            "i = {}; 100 in {} ns; {} ns/hash; {} ms/hash",
            i, total, total / N as u128, ms_hash,
        );

        if ms_hash >= 100.0 {
            println!("Smoketested up to i = {}, which took longer that 100ms/hash", i);
            break;
        }
    }
}
