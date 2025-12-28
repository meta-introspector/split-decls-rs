macro_rules! deps {
    () => {
        Fingerprint!();
    };
}

macro_rules! impl_39 {
    () => {
        deps!();
        impl < E : Encoder > Encodable < E > for Fingerprint { # [inline] fn encode (& self , s : & mut E) { s . emit_raw_bytes (& self . to_le_bytes ()) ; } }
    };
}

impl_39!();