macro_rules! deps {
    () => {
        TargetTuple!();
    };
}

macro_rules! impl_548 {
    () => {
        deps!();
        impl < S : Encoder > Encodable < S > for TargetTuple { fn encode (& self , s : & mut S) { match self { TargetTuple :: TargetTuple (tuple) => { s . emit_u8 (0) ; s . emit_str (tuple) ; } TargetTuple :: TargetJson { path_for_rustdoc : _ , tuple , contents } => { s . emit_u8 (1) ; s . emit_str (tuple) ; s . emit_str (contents) ; } } } }
    };
}

impl_548!();