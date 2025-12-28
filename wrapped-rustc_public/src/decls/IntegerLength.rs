macro_rules! IntegerLength {
    () => {
        # [doc = " Enum representing the existing integer lengths."] # [derive (Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash , Debug , Serialize)] pub enum IntegerLength { I8 , I16 , I32 , I64 , I128 , }
    };
}

IntegerLength!()