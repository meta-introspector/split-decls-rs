macro_rules! deps {
    () => {
        FromRawReprError!();
        AlignRepr!();
        UnsupportedReprError!();
        RawRepr!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        impl < Pcked : With < NonZeroU32 > > TryFrom < RawRepr > for AlignRepr < Pcked > { type Error = FromRawReprError < UnsupportedReprError > ; fn try_from (raw : RawRepr) -> Result < AlignRepr < Pcked > , FromRawReprError < UnsupportedReprError > > { use RawRepr :: * ; match raw { Packed | PackedN (_) => Pcked :: try_with_or (| | match raw { Packed => Ok (NonZeroU32 :: new (1) . unwrap ()) , PackedN (n) => Ok (n) , U8 | U16 | U32 | U64 | U128 | Usize | I8 | I16 | I32 | I64 | I128 | Isize | Transparent | C | Rust | Align (_) => Err (UnsupportedReprError) , } , UnsupportedReprError ,) . map (AlignRepr :: Packed) . map_err (FromRawReprError :: Err) , Align (n) => Ok (AlignRepr :: Align (n)) , U8 | U16 | U32 | U64 | U128 | Usize | I8 | I16 | I32 | I64 | I128 | Isize | Transparent | C | Rust => Err (FromRawReprError :: None) , } } }
    };
}

impl_45!()