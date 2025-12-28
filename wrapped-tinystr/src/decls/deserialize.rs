macro_rules! deps {
    () => {
        TinyAsciiStr!();
        UnvalidatedTinyAsciiStr!();
    };
}

macro_rules! deserialize {
    () => {
        deps!();
        macro_rules ! deserialize { ($ size : literal) => { # [cfg (feature = "serde")] impl <'de , 'a > serde_core :: Deserialize <'de > for UnvalidatedTinyAsciiStr <$ size > where 'de : 'a , { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : serde_core :: Deserializer <'de >, { if deserializer . is_human_readable () { Ok (TinyAsciiStr :: deserialize (deserializer) ?. to_unvalidated ()) } else { Ok (Self (< [u8 ; $ size] >:: deserialize (deserializer) ?)) } } } } ; }
    };
}

deserialize!()