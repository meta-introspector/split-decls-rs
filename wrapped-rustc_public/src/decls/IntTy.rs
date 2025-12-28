macro_rules! IntTy {
    () => {
        # [derive (Clone , Copy , Debug , PartialEq , Eq , Serialize)] pub enum IntTy { Isize , I8 , I16 , I32 , I64 , I128 , }
    };
}

IntTy!()