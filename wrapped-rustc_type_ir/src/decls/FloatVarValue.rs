macro_rules! FloatVarValue {
    () => {
        # [derive (Clone , Copy , PartialEq , Eq , Debug)] pub enum FloatVarValue { Unknown , Known (FloatTy) , }
    };
}

FloatVarValue!()