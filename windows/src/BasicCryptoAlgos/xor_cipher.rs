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
pub fn xor_decrypt(data:&mut [u8],key:String){
     //converting the String into bytes so it can be xored with the same datatype.
    for i in 0..data.len(){
        data[i] ^=key[i%key.len()];
    }

}
pub fn xor_encrypt(data: &[u8],key: &str) -> String{
    let ciphertext = xor(data,key);
    let hex_str: Vec<String> = ciphertext.iter().map(|&x| format!("{:02x}",x)).collect();
    //iter() is to iterate , map is to apply filters and collect is to collect them all into a single datatype.

    //FOR CONVERSION OF DATATYPES HERE WE CAN EASILY USE THE FORMAT METHOD
    format!("{{ 0x{} }};",hex_str.join(", 0x"))
    //the format is used to format the first hex number and then the ", 0x" stores the every next hex number with 0x as a prefix value to the hex number inside the hex string
      
}