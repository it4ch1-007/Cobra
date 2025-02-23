pub fn xor(data: &[u8],key: &str) -> Vec<u8>{
    let key_bytes = key.as_bytes();//this converts the key (a string) into the bytes equivalent bytestring
    let mut output = Vec::with_capacity(data.len());
    //this makes a vector of a particular length
    //It improves performance


    //Iterated over the whole array and make a tuple of 
    for(i, &byte) in data.iter().enumerate()
    {
        let current_key = key_bytes[i%key_bytes.len()];
        output.push(byte^current_key.wrapping_add(i as u8));
        //THE CURRENT_KEY IS ACTUALLY A CHARACTER OF THE WHOLE KEY AT AN INDEX.
    }
    output
    //the i is the index variable and the &byte stores the reference to the element at that index.

}
