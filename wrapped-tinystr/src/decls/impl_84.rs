macro_rules! deps {
    () => {
        TinyAsciiStr!();
    };
}

macro_rules! impl_84 {
    () => {
        deps!();
        unsafe impl < const N : usize > ULE for TinyAsciiStr < N > { # [inline] fn validate_bytes (bytes : & [u8]) -> Result < () , UleError > { if bytes . len () % N != 0 { return Err (UleError :: length :: < Self > (bytes . len ())) ; } for chunk in bytes . chunks_exact (N) { let _ = TinyAsciiStr :: < N > :: try_from_utf8_inner (chunk , true) . map_err (| _ | UleError :: parse :: < Self > ()) ? ; } Ok (()) } }
    };
}

impl_84!();