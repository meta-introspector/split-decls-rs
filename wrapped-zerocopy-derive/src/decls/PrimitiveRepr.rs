macro_rules! PrimitiveRepr {
    () => {
        # [doc = " `repr(Int)`"] # [derive (Copy , Clone)] # [cfg_attr (test , derive (Debug , Eq , PartialEq))] pub (crate) enum PrimitiveRepr { U8 , U16 , U32 , U64 , U128 , Usize , I8 , I16 , I32 , I64 , I128 , Isize , }
    };
}

PrimitiveRepr!()