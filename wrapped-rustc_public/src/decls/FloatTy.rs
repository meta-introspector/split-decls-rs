macro_rules! FloatTy {
    () => {
        # [derive (Clone , Copy , Debug , PartialEq , Eq , Serialize)] pub enum FloatTy { F16 , F32 , F64 , F128 , }
    };
}

FloatTy!();