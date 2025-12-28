macro_rules! impl_322 {
    () => {
        impl < S : Encoder > Encodable < S > for BytePos { fn encode (& self , s : & mut S) { s . emit_u32 (self . 0) ; } }
    };
}

impl_322!()