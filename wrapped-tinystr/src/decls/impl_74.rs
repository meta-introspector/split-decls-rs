macro_rules! deps {
    () => {
        TinyAsciiStr!();
        TinyAsciiStrVisitor!();
    };
}

macro_rules! impl_74 {
    () => {
        deps!();
        impl < 'de , const N : usize > Visitor < 'de > for TinyAsciiStrVisitor < N > { type Value = TinyAsciiStr < N > ; fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { write ! (formatter , "a TinyAsciiStr<{N}>") } # [inline] fn visit_seq < A > (self , mut seq : A) -> Result < Self :: Value , A :: Error > where A : SeqAccess < 'de > , { let mut bytes = [0u8 ; N] ; let mut zeroes = false ; for out in & mut bytes . iter_mut () . take (N) { let byte = seq . next_element () ? . ok_or_else (| | Error :: invalid_length (N , & self)) ? ; if byte == 0 { zeroes = true ; } else if zeroes { return Err (Error :: custom ("TinyAsciiStr cannot contain null bytes")) ; } if byte >= 0x80 { return Err (Error :: custom ("TinyAsciiStr cannot contain non-ascii bytes")) ; } * out = byte ; } Ok (unsafe { TinyAsciiStr :: from_utf8_unchecked (bytes) }) } }
    };
}

impl_74!();