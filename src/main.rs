use libreauth::{
    hash::HashFunction::Sha3_512,
    oath::{TOTPBuilder, TOTP},
};

const OTP_KEY: &'static str = "1kozlvoQpBlIPgEN";

fn build_generator() -> TOTP {
    TOTPBuilder::new()
        .ascii_key(OTP_KEY)
        .hash_function(Sha3_512)
        .period(10)
        .finalize()
        .expect("Could not initialize builder")
}

fn create_totp(generator: &TOTP) {
    let code = generator.generate();

    println!("{:?}", code);
}

fn validate_totp(generator: &TOTP, code: &str) {
    let valid = generator.is_valid(code);

    println!("{:?}", valid);
}

fn main() {
    println!("Hello, world!");
    let generator = build_generator();
    create_totp(&generator);
    validate_totp(&generator, "447017");
}
