macro_rules! deps {
    () => {
        Error!();
        DeTable!();
    };
}

macro_rules! validate_struct_keys {
    () => {
        deps!();
        pub (crate) fn validate_struct_keys (table : & DeTable < '_ > , fields : & 'static [& 'static str] ,) -> Result < () , Error > { let extra_fields = table . keys () . filter_map (| key | { if ! fields . contains (& key . get_ref () . as_ref ()) { Some (key . clone ()) } else { None } }) . collect :: < Vec < _ > > () ; if extra_fields . is_empty () { Ok (()) } else { Err (Error :: custom (format ! ("unexpected keys in table: {}, available keys: {}" , extra_fields . iter () . map (| k | k . get_ref () . as_ref ()) . collect ::< Vec < _ >> () . join (", ") , fields . join (", ") ,) , Some (extra_fields [0] . span ()) ,)) } }
    };
}

validate_struct_keys!()