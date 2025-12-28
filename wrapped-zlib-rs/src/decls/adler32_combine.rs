macro_rules! adler32_combine {
    () => {
        pub fn adler32_combine (adler1 : u32 , adler2 : u32 , len2 : u64) -> u32 { const BASE : u64 = self :: BASE as u64 ; let rem = len2 % BASE ; let adler1 = adler1 as u64 ; let adler2 = adler2 as u64 ; let mut sum1 = adler1 & 0xffff ; let mut sum2 = rem * sum1 ; sum2 %= BASE ; sum1 += (adler2 & 0xffff) + BASE - 1 ; sum2 += ((adler1 >> 16) & 0xffff) + ((adler2 >> 16) & 0xffff) + BASE - rem ; if sum1 >= BASE { sum1 -= BASE ; } if sum1 >= BASE { sum1 -= BASE ; } if sum2 >= (BASE << 1) { sum2 -= BASE << 1 ; } if sum2 >= BASE { sum2 -= BASE ; } (sum1 | (sum2 << 16)) as u32 }
    };
}

adler32_combine!();