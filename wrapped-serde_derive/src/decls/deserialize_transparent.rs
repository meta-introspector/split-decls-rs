macro_rules! deps {
    () => {
        Fragment!();
        Default!();
        Data!();
        Container!();
        Parameters!();
    };
}

macro_rules! deserialize_transparent {
    () => {
        deps!();
        # [doc = " Generates `Deserialize::deserialize` body for a type with `#[serde(transparent)]` attribute"] fn deserialize_transparent (cont : & Container , params : & Parameters) -> Fragment { let fields = match & cont . data { Data :: Struct (_ , fields) => fields , Data :: Enum (_) => unreachable ! () , } ; let this_value = & params . this_value ; let transparent_field = fields . iter () . find (| f | f . attrs . transparent ()) . unwrap () ; let path = match transparent_field . attrs . deserialize_with () { Some (path) => quote ! (# path) , None => { let span = transparent_field . original . span () ; quote_spanned ! (span => _serde :: Deserialize :: deserialize) } } ; let assign = fields . iter () . map (| field | { let member = & field . member ; if ptr :: eq (field , transparent_field) { quote ! (# member : __transparent) } else { let value = match field . attrs . default () { attr :: Default :: Default => quote ! (_serde ::# private :: Default :: default ()) , attr :: Default :: Path (path) => quote_spanned ! (path . span () => # path ()) , attr :: Default :: None => quote ! (_serde ::# private :: PhantomData) , } ; quote ! (# member : # value) } }) ; quote_block ! { _serde ::# private :: Result :: map (# path (__deserializer) , | __transparent | # this_value { # (# assign) ,* }) } }
    };
}

deserialize_transparent!();