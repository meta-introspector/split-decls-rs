macro_rules! deps {
    () => {
        Tag!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        impl Tag { # [doc = "\n    Create a new tag from a static string value.\n    "] pub const fn new (data : & 'static str) -> Self { const fn compute_id (bytes : & [u8]) -> u64 { const K : u64 = 0x517cc1b727220a95u64 ; let mut hash = 0u64 ; let mut b = 0 ; while b + 8 <= bytes . len () { let i = [bytes [b + 0] , bytes [b + 1] , bytes [b + 2] , bytes [b + 3] , bytes [b + 4] , bytes [b + 5] , bytes [b + 6] , bytes [b + 7] ,] ; let i = u64 :: from_ne_bytes (i) ; hash = (hash . rotate_left (5) ^ i) . wrapping_mul (K) ; b += 8 ; } if b + 4 <= bytes . len () { let i = [bytes [b + 0] , bytes [b + 1] , bytes [b + 2] , bytes [b + 3]] ; let i = u32 :: from_ne_bytes (i) as u64 ; hash = (hash . rotate_left (5) ^ i) . wrapping_mul (K) ; b += 4 ; } if b + 2 <= bytes . len () { let i = [bytes [b + 0] , bytes [b + 1]] ; let i = u16 :: from_ne_bytes (i) as u64 ; hash = (hash . rotate_left (5) ^ i) . wrapping_mul (K) ; b += 2 ; } if b + 1 <= bytes . len () { let i = bytes [b + 0] as u64 ; hash = (hash . rotate_left (5) ^ i) . wrapping_mul (K) ; } hash } Tag { id : compute_id (data . as_bytes ()) , data , } } # [inline (always)] const fn cloned (& self) -> Tag { Tag { id : self . id , data : self . data , } } }
    };
}

impl_24!();