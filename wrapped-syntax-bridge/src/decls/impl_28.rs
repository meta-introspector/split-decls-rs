macro_rules! deps {
    () => {
        SrcToken!();
        StaticRawConverter!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl < S : Copy > SrcToken < StaticRawConverter < '_ , S > , S > for usize { fn kind (& self , ctx : & StaticRawConverter < '_ , S >) -> SyntaxKind { ctx . lexed . kind (* self) } fn to_char (& self , ctx : & StaticRawConverter < '_ , S >) -> Option < char > { ctx . lexed . text (* self) . chars () . next () } fn to_text (& self , ctx : & StaticRawConverter < '_ , S >) -> SmolStr { ctx . lexed . text (* self) . into () } }
    };
}

impl_28!()