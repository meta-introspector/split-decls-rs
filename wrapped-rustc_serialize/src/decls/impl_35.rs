macro_rules! deps {
    () => {
        Encodable!();
        Encoder!();
    };
}

macro_rules! impl_35 {
    () => {
        deps!();
        impl < S : Encoder , T1 : Encodable < S > , T2 : Encodable < S > > Encodable < S > for Result < T1 , T2 > { fn encode (& self , s : & mut S) { match * self { Ok (ref v) => { s . emit_u8 (0) ; v . encode (s) ; } Err (ref v) => { s . emit_u8 (1) ; v . encode (s) ; } } } }
    };
}

impl_35!()