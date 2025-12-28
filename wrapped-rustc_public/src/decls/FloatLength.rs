macro_rules! FloatLength {
    () => {
        # [doc = " Enum representing the existing float lengths."] # [derive (Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash , Debug , Serialize)] pub enum FloatLength { F16 , F32 , F64 , F128 , }
    };
}

FloatLength!();