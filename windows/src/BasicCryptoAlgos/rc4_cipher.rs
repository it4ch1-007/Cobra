struct RC4 {
    s_box: [u8; 256],
    x: usize,
    y: usize,
}

impl RC4 {
    fn new(key: &[u8]) -> Self {
        let mut s_box = [0u8; 256];
        let mut j: usize = 0;

        // Initialize the S-box
        for i in 0..256 {
            s_box[i] = i as u8;
        }

        // Key scheduling algorithm
        for i in 0..256 {
            j = (j + s_box[i] as usize + key[i % key.len()] as usize) % 256;
            s_box.swap(i, j);
        }

        RC4 {
            s_box,
            x: 0,
            y: 0,
        }
    }

    fn next_byte(&mut self) -> u8 {
        self.x = (self.x + 1) % 256;
        self.y = (self.y + self.s_box[self.x] as usize) % 256;
        self.s_box.swap(self.x, self.y);
        self.s_box[(self.s_box[self.x] as usize + self.s_box[self.y] as usize) % 256]
    }

    fn encrypt(&mut self, data: &[u8]) -> Vec<u8> {
        data.iter().map(|&byte| byte ^ self.next_byte()).collect()
    }
}

pub fn fnrc4(input: &str) -> Vec<u8> {
    let key = b"";  // Key should be between 5 to 16 bytes for RC4
    let data = input.as_bytes();

    let mut rc4 = RC4::new(key);
    let encrypted_data = rc4.encrypt(data);

    encrypted_data
}