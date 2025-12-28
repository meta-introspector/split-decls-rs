macro_rules! deps {
    () => {
        ErrorSample!();
    };
}

macro_rules! ErrorCollection {
    () => {
        deps!();
        # [derive (Debug , Serialize , Deserialize , Default)] pub struct ErrorCollection { pub errors : Vec < ErrorSample > , }
    };
}

ErrorCollection!()