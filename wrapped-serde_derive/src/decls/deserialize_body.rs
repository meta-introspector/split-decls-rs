macro_rules! deps {
    () => {
        Parameters!();
        StructForm!();
        TupleForm!();
        Style!();
        Fragment!();
        Container!();
        Identifier!();
        Data!();
    };
}

macro_rules! deserialize_body {
    () => {
        deps!();
        fn deserialize_body (cont : & Container , params : & Parameters) -> Fragment { if cont . attrs . transparent () { deserialize_transparent (cont , params) } else if let Some (type_from) = cont . attrs . type_from () { deserialize_from (type_from) } else if let Some (type_try_from) = cont . attrs . type_try_from () { deserialize_try_from (type_try_from) } else if let attr :: Identifier :: No = cont . attrs . identifier () { match & cont . data { Data :: Enum (variants) => enum_ :: deserialize (params , variants , & cont . attrs) , Data :: Struct (Style :: Struct , fields) => { struct_ :: deserialize (params , fields , & cont . attrs , StructForm :: Struct) } Data :: Struct (Style :: Tuple , fields) | Data :: Struct (Style :: Newtype , fields) => { tuple :: deserialize (params , fields , & cont . attrs , TupleForm :: Tuple) } Data :: Struct (Style :: Unit , _) => unit :: deserialize (params , & cont . attrs) , } } else { match & cont . data { Data :: Enum (variants) => identifier :: deserialize_custom (params , variants , & cont . attrs) , Data :: Struct (_ , _) => unreachable ! ("checked in serde_derive_internals") , } } }
    };
}

deserialize_body!();