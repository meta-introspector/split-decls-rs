macro_rules! impl_101 {
    () => {
        impl < S : Encoder > Encodable < S > for RelativeBytePos { fn encode (& self , s : & mut S) { s . emit_u32 (self . 0) ; } }
    };
}

impl_101!()