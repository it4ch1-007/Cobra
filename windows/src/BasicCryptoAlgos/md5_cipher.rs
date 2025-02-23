use std::convert::TryInto;

#[derive(Clone, Copy)]
struct MD5 {
    state: [u32; 4],
}

impl MD5 {
    fn new() -> Self {
        MD5 {
            state: [0x67452301, 0xefcdab89, 0x98badcfe, 0x10325476],
        }
    }

    fn process_block(&mut self, block: &[u8]) {
        // Ensure block is exactly 64 bytes
        assert_eq!(block.len(), 64, "Block must be 64 bytes");

        // Prepare message schedule
        let mut words = [0u32; 16];
        for (i, chunk) in block.chunks_exact(4).enumerate() {
            words[i] = u32::from_le_bytes(chunk.try_into().unwrap());
        }

        // Initialize working variables
        let mut a = self.state[0];
        let mut b = self.state[1];
        let mut c = self.state[2];
        let mut d = self.state[3];

        // Define round functions with explicit type annotations
        let f = |x: u32, y: u32, z: u32| -> u32 { (x & y) | (!x & z) };
        let g = |x: u32, y: u32, z: u32| -> u32 { (x & z) | (y & !z) };
        let h = |x: u32, y: u32, z: u32| -> u32 { x ^ y ^ z };
        let i_func = |x: u32, y: u32, z: u32| -> u32 { y ^ (x | !z) };

        // Predefined constants and rotation amounts
        const CONSTANTS: [u32; 64] = [
            0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee, 0xf57c0faf, 0x4787c62a, 0xa8304613, 0xfd469501,
            0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be, 0x6b901122, 0xfd987193, 0xa679438e, 0x49b40821,
            0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa, 0xd62f105d, 0x02441453, 0xd8a1e681, 0xe7d3fbc8,
            0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed, 0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a,
            0xfffa3942, 0x8771f681, 0x6d9d6122, 0xfde5380c, 0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70,
            0x289b7ec6, 0xeaa127fa, 0xd4ef3085, 0x04881d05, 0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665,
            0xf4292244, 0x432aff97, 0xab9423a7, 0xfc93a039, 0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
            0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1, 0xf7537e82, 0xbd3af235, 0x2ad7d2bb, 0xeb86d391,
        ];

        const SHIFTS: [u32; 64] = [
            7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22,
            5,  9, 14, 20, 5,  9, 14, 20, 5,  9, 14, 20, 5,  9, 14, 20,
            4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23,
            6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21
        ];

        // Main MD5 rounds
        for i in 0..64 {
            let (f_result, g) = match i {
                0..=15 => (f(b, c, d), i),
                16..=31 => (g(b, c, d), (1 + 5 * i) % 16),
                32..=47 => (h(b, c, d), (5 + 3 * i) % 16),
                _ => (i_func(b, c, d), (7 * i) % 16)
            };

            let temp = d;
            d = c;
            c = b;
            b = b.wrapping_add(
                a.wrapping_add(f_result)
                  .wrapping_add(CONSTANTS[i])
                  .wrapping_add(words[g])
                  .rotate_left(SHIFTS[i])
            );
            a = temp;
        }

        // Update state
        self.state[0] = self.state[0].wrapping_add(a);
        self.state[1] = self.state[1].wrapping_add(b);
        self.state[2] = self.state[2].wrapping_add(c);
        self.state[3] = self.state[3].wrapping_add(d);
    }

    fn finalize(&mut self, message: &[u8], original_len: u64) -> Vec<u8> {
        // Padding
        let mut padded = message.to_vec();
        padded.push(0x80);
        while (padded.len() + 8) % 64 != 0 {
            padded.push(0);
        }

        // Append original length in bits
        let bit_len = original_len * 8;
        padded.extend_from_slice(&bit_len.to_le_bytes());

        // Process blocks
        for chunk in padded.chunks(64) {
            self.process_block(chunk);
        }

        // Convert final state to byte array
        self.state
            .iter()
            .flat_map(|&x| x.to_le_bytes().to_vec())
            .collect()
    }

    fn hash(message: &[u8]) -> Vec<u8> {
        let mut md5 = MD5::new();
        md5.finalize(message, message.len() as u64)
    }
}

pub fn fn_md5(input:&str) -> String {
    let hash = MD5::hash(input.as_bytes());

    let result = hash.iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>();
    result
}