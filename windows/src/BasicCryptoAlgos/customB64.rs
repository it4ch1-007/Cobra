use base64::{
    alphabet::Alphabet,
    engine::general_purpose::{GeneralPurpose, GeneralPurposeConfig},
    Engine as _,
};



// Create a custom engine
fn custom_base64_engine() -> GeneralPurpose {
    let CUSTOM_ALPHABET: &str = "/+9876543210ZYXWVUTSRQPONMLKJIHGFEDCBAzyxwvutsrqponmlkjihgfedcba";
    let CUSTOM_ALPHABET_MOD: Alphabet = Alphabet::new(CUSTOM_ALPHABET).expect("Error");
    GeneralPurpose::new(
        &CUSTOM_ALPHABET_MOD,
        GeneralPurposeConfig::new(),
    )
}

pub fn custom_base64_decode(input: &[u8]) -> String {
    let engine = custom_base64_engine();
    
    // Decode the Base64 bytes to raw bytes
    let decoded_bytes = engine
        .decode(input)
        .expect("Invalid input for custom Base64 decoding");

    // Convert the bytes to a UTF-8 string
    String::from_utf8(decoded_bytes).expect("Decoded bytes are not valid UTF-8")
}