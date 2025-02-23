pub fn xor(data: &[u8],key: &str) -> Vec<u8>{
    let key_bytes = key.as_bytes();
    let mut output = Vec::with_capacity(data.len());
    for(i, &byte) in data.iter().enumerate()
    {
        let current_key = key_bytes[i%key_bytes.len()];
        output.push(byte^current_key.wrapping_add(i as u8));
    }
    output

}
