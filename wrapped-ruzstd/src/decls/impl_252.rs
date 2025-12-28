macro_rules! deps {
    () => {
        SuffixStore!();
    };
}

macro_rules! impl_252 {
    () => {
        deps!();
        impl SuffixStore { fn with_capacity (capacity : usize) -> Self { Self { slots : alloc :: vec ! [None ; capacity] , len_log : capacity . ilog2 () , } } # [inline (always)] fn insert (& mut self , suffix : & [u8] , idx : usize) { let key = self . key (suffix) ; self . slots [key] = Some (NonZeroUsize :: new (idx + 1) . unwrap ()) ; } # [inline (always)] fn contains_key (& self , suffix : & [u8]) -> bool { let key = self . key (suffix) ; self . slots [key] . is_some () } # [inline (always)] fn get (& self , suffix : & [u8]) -> Option < usize > { let key = self . key (suffix) ; self . slots [key] . map (| x | < NonZeroUsize as Into < usize > > :: into (x) - 1) } # [inline (always)] fn key (& self , suffix : & [u8]) -> usize { let s0 = suffix [0] as u64 ; let s1 = suffix [1] as u64 ; let s2 = suffix [2] as u64 ; let s3 = suffix [3] as u64 ; let s4 = suffix [4] as u64 ; const POLY : u64 = 0xCF3BCCDCABu64 ; let s0 = (s0 << 24) . wrapping_mul (POLY) ; let s1 = (s1 << 32) . wrapping_mul (POLY) ; let s2 = (s2 << 40) . wrapping_mul (POLY) ; let s3 = (s3 << 48) . wrapping_mul (POLY) ; let s4 = (s4 << 56) . wrapping_mul (POLY) ; let index = s0 ^ s1 ^ s2 ^ s3 ^ s4 ; let index = index >> (64 - self . len_log) ; index as usize % self . slots . len () } }
    };
}

impl_252!();