macro_rules! deps {
    () => {
        UnvalidatedTinyAsciiStr!();
    };
}

macro_rules! impl_88 {
    () => {
        deps!();
        unsafe impl < const N : usize > ULE for UnvalidatedTinyAsciiStr < N > { # [inline] fn validate_bytes (bytes : & [u8]) -> Result < () , UleError > { if bytes . len () % N != 0 { return Err (UleError :: length :: < Self > (bytes . len ())) ; } Ok (()) } }
    };
}

impl_88!();