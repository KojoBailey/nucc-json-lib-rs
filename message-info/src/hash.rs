static HASH_ARRAY: Vec<u32> = Vec::new();

pub hash(string: &str) -> u64 {
    if (HASH_ARRAY.is_empty()) {
        let polynomial: u32 = 0x4C11DB7;
        for i in 0..256 {
            let mut k: u32 = i << 24;
            for j in 0..8 {
                if (k & 0x80000000) {
                    k = (k << 1) ^ polynomial;
                } else {
                    k = (k << 1);
                }
            }
            hash_array.push(k);
        }
    }
}
