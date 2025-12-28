macro_rules! deps {
    () => {
        InlineTable!();
    };
}

macro_rules! encode_table {
    () => {
        deps!();
        pub (crate) fn encode_table (this : & InlineTable , mut buf : & mut dyn Write , input : Option < & str > , default_decor : (& str , & str) ,) -> Result { let decor = this . decor () ; decor . prefix_encode (buf , input , default_decor . 0) ? ; buf . open_inline_table () ? ; this . preamble () . encode_with_default (buf , input , "") ? ; let children = this . get_values () ; let len = children . len () ; for (i , (key_path , value)) in children . into_iter () . enumerate () { if i != 0 { buf . val_sep () ? ; } let inner_decor = if i == len - 1 { DEFAULT_TRAILING_VALUE_DECOR } else { DEFAULT_VALUE_DECOR } ; encode_key_path_ref (& key_path , buf , input , DEFAULT_INLINE_KEY_DECOR) ? ; buf . keyval_sep () ? ; encode_value (value , buf , input , inner_decor) ? ; } buf . close_inline_table () ? ; decor . suffix_encode (buf , input , default_decor . 1) ? ; Ok (()) }
    };
}

encode_table!()