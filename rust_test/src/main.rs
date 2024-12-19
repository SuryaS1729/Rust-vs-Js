use std::time::Instant;

fn main() {
    let start = Instant::now();

    let mut sum: u64 = 0;
    for i in 0..100_000_000 {
        sum += i;
    }

    let duration = start.elapsed();
    println!("Rust execution time is: {:?}", duration);
    println!("Sum: {}", sum);
}

