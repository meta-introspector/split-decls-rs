macro_rules! deps {
    () => {
        Expr!();
        Container!();
        Parameters!();
        Field!();
        Fragment!();
        Default!();
    };
}

macro_rules! deserialize_seq_in_place {
    () => {
        deps!();
        # [cfg (feature = "deserialize_in_place")] fn deserialize_seq_in_place (params : & Parameters , fields : & [Field] , cattrs : & attr :: Container , expecting : & str ,) -> Fragment { let deserialized_count = fields . iter () . filter (| field | ! field . attrs . skip_deserializing ()) . count () ; let expecting = if deserialized_count == 1 { format ! ("{} with 1 element" , expecting) } else { format ! ("{} with {} elements" , expecting , deserialized_count) } ; let expecting = cattrs . expecting () . unwrap_or (& expecting) ; let mut index_in_seq = 0usize ; let write_values = fields . iter () . map (| field | { let member = & field . member ; if field . attrs . skip_deserializing () { let default = Expr (expr_is_missing (field , cattrs)) ; quote ! { self . place .# member = # default ; } } else { let value_if_none = expr_is_missing_seq (Some (quote ! (self . place .# member =)) , index_in_seq , field , cattrs , expecting) ; let write = match field . attrs . deserialize_with () { None => { quote ! { if let _serde ::# private :: None = _serde :: de :: SeqAccess :: next_element_seed (& mut __seq , _serde ::# private :: de :: InPlaceSeed (& mut self . place .# member)) ? { # value_if_none ; } } } Some (path) => { let (wrapper , wrapper_ty) = wrap_deserialize_field_with (params , field . ty , path) ; quote ! ({ # wrapper match _serde :: de :: SeqAccess :: next_element ::<# wrapper_ty > (& mut __seq) { _serde ::# private :: Ok (_serde ::# private :: Some (__wrap)) => { self . place .# member = __wrap . value ; } _serde ::# private :: Ok (_serde ::# private :: None) => { # value_if_none ; } _serde ::# private :: Err (__err) => { return _serde ::# private :: Err (__err) ; } } }) } } ; index_in_seq += 1 ; write } }) ; let this_type = & params . this_type ; let (_ , ty_generics , _) = params . generics . split_for_impl () ; let let_default = match cattrs . default () { attr :: Default :: Default => Some (quote ! (let __default : # this_type # ty_generics = _serde ::# private :: Default :: default () ;)) , attr :: Default :: Path (path) => Some (quote_spanned ! (path . span () => let __default : # this_type # ty_generics = # path () ;)) , attr :: Default :: None => { None } } ; quote_block ! { # let_default # (# write_values) * _serde ::# private :: Ok (()) } }
    };
}

deserialize_seq_in_place!();