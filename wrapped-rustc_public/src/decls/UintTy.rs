macro_rules! UintTy {
    () => {
        # [derive (Clone , Copy , Debug , PartialEq , Eq , Serialize)] pub enum UintTy { Usize , U8 , U16 , U32 , U64 , U128 , }
    };
}

UintTy!();