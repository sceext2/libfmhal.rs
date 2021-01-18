#[cfg(feature = "shal")]
mod shal;

#[test]
fn allow_use_std_in_test() {
    println!("Hello, world !");
}
