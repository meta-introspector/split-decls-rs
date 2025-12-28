macro_rules! deps {
    () => {
        RawRepr!();
        CompoundRepr!();
        FromRawReprError!();
        PrimitiveRepr!();
        UnsupportedReprError!();
    };
}

macro_rules! impl_44 {
    () => {
        deps!();
        impl < Prim : With < PrimitiveRepr > > TryFrom < RawRepr > for CompoundRepr < Prim > { type Error = FromRawReprError < UnsupportedReprError > ; fn try_from (raw : RawRepr ,) -> Result < CompoundRepr < Prim > , FromRawReprError < UnsupportedReprError > > { use RawRepr :: * ; match raw { C => Ok (CompoundRepr :: C) , Rust => Ok (CompoundRepr :: Rust) , raw @ (U8 | U16 | U32 | U64 | U128 | Usize | I8 | I16 | I32 | I64 | I128 | Isize) => { Prim :: try_with_or (| | match raw { U8 => Ok (PrimitiveRepr :: U8) , U16 => Ok (PrimitiveRepr :: U16) , U32 => Ok (PrimitiveRepr :: U32) , U64 => Ok (PrimitiveRepr :: U64) , U128 => Ok (PrimitiveRepr :: U128) , Usize => Ok (PrimitiveRepr :: Usize) , I8 => Ok (PrimitiveRepr :: I8) , I16 => Ok (PrimitiveRepr :: I16) , I32 => Ok (PrimitiveRepr :: I32) , I64 => Ok (PrimitiveRepr :: I64) , I128 => Ok (PrimitiveRepr :: I128) , Isize => Ok (PrimitiveRepr :: Isize) , Transparent | C | Rust | Align (_) | PackedN (_) | Packed => { Err (UnsupportedReprError) } } , UnsupportedReprError ,) . map (CompoundRepr :: Primitive) . map_err (FromRawReprError :: Err) } Transparent | Align (_) | PackedN (_) | Packed => Err (FromRawReprError :: None) , } } }
    };
}

impl_44!()