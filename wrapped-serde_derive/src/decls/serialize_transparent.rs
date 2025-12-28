macro_rules! deps {
    () => {
        Container!();
        Data!();
        Parameters!();
        Fragment!();
    };
}

macro_rules! serialize_transparent {
    () => {
        deps!();
        fn serialize_transparent (cont : & Container , params : & Parameters) -> Fragment { let fields = match & cont . data { Data :: Struct (_ , fields) => fields , Data :: Enum (_) => unreachable ! () , } ; let self_var = & params . self_var ; let transparent_field = fields . iter () . find (| f | f . attrs . transparent ()) . unwrap () ; let member = & transparent_field . member ; let path = match transparent_field . attrs . serialize_with () { Some (path) => quote ! (# path) , None => { let span = transparent_field . original . span () ; quote_spanned ! (span => _serde :: Serialize :: serialize) } } ; quote_block ! { # path (&# self_var .# member , __serializer) } }
    };
}

serialize_transparent!()