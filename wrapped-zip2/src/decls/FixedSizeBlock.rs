macro_rules! deps {
    () => {
        Pod!();
        ZipError!();
        Magic!();
        ZipResult!();
    };
}

macro_rules! FixedSizeBlock {
    () => {
        deps!();
        pub (crate) trait FixedSizeBlock : Pod { const MAGIC : Magic ; fn magic (self) -> Magic ; const WRONG_MAGIC_ERROR : ZipError ; # [allow (clippy :: wrong_self_convention)] fn from_le (self) -> Self ; fn parse < R : Read > (reader : & mut R) -> ZipResult < Self > { let mut block = Self :: zeroed () ; reader . read_exact (block . as_bytes_mut ()) ? ; let block = Self :: from_le (block) ; if block . magic () != Self :: MAGIC { return Err (Self :: WRONG_MAGIC_ERROR) ; } Ok (block) } fn to_le (self) -> Self ; fn write < T : Write > (self , writer : & mut T) -> ZipResult < () > { let block = self . to_le () ; writer . write_all (block . as_bytes ()) ? ; Ok (()) } }
    };
}

FixedSizeBlock!()