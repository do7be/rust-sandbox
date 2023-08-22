#[cfg(not(feature = "beta"))]
pub fn is_beta() -> &'static str {
    "Not Beta"
}

#[cfg(feature = "beta")]
pub fn is_beta() -> &'static str {
    "Beta"
}

fn main() {
    // cargo run --bin feature_flag だと "Not Beta"
    // cargo run --bin feature_flag --features beta だと "Beta"
    println!("{}", is_beta());
}
