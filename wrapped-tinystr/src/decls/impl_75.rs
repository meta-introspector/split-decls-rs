macro_rules! deps {
    () => {
        TinyAsciiStrVisitor!();
        TinyAsciiStr!();
    };
}

macro_rules! impl_75 {
    () => {
        deps!();
        impl < 'de , const N : usize > Deserialize < 'de > for TinyAsciiStr < N > { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { if deserializer . is_human_readable () { struct HumanVisitor < const N : usize > ; impl < 'de , const M : usize > Visitor < 'de > for HumanVisitor < M > { type Value = TinyAsciiStr < M > ; fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { write ! (formatter , "a TinyAsciiStr<{M}>") } fn visit_str < E > (self , v : & str) -> Result < Self :: Value , E > where E : Error , { TinyAsciiStr :: try_from_str (v) . map_err (| _ | Error :: custom ("invalid str")) } } deserializer . deserialize_str (HumanVisitor :: < N >) } else { deserializer . deserialize_tuple (N , TinyAsciiStrVisitor :: < N > :: new ()) } } }
    };
}

impl_75!();