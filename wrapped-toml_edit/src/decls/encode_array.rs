macro_rules! deps {
    () => {
        Array!();
    };
}

macro_rules! encode_array {
    () => {
        deps!();
        pub (crate) fn encode_array (this : & Array , mut buf : & mut dyn Write , input : Option < & str > , default_decor : (& str , & str) ,) -> Result { let decor = this . decor () ; decor . prefix_encode (buf , input , default_decor . 0) ? ; buf . open_array () ? ; for (i , elem) in this . iter () . enumerate () { let inner_decor ; if i == 0 { inner_decor = DEFAULT_LEADING_VALUE_DECOR ; } else { inner_decor = DEFAULT_VALUE_DECOR ; buf . val_sep () ? ; } encode_value (elem , buf , input , inner_decor) ? ; } if this . trailing_comma () && ! this . is_empty () { buf . val_sep () ? ; } this . trailing () . encode_with_default (buf , input , "") ? ; buf . close_array () ? ; decor . suffix_encode (buf , input , default_decor . 1) ? ; Ok (()) }
    };
}

encode_array!();