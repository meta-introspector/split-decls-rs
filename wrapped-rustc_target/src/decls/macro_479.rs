macro_rules! macro_479 {
    () => {
        crate :: target_spec_enum ! { # [derive (Encodable , Decodable , HashStable_Generic)] pub enum PanicStrategy { Unwind = "unwind" , Abort = "abort" , } parse_error_type = "panic strategy" ; }
    };
}

macro_479!()