macro_rules! IntVarValue {
    () => {
        # [derive (Clone , Copy , PartialEq , Eq , Debug)] pub enum IntVarValue { Unknown , IntType (IntTy) , UintType (UintTy) , }
    };
}

IntVarValue!()