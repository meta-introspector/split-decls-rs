macro_rules! deps {
    () => {
        Item!();
        Array!();
    };
}

macro_rules! decorate_array {
    () => {
        deps!();
        fn decorate_array (array : & mut Array) { for (i , value) in array . values . iter_mut () . filter_map (Item :: as_value_mut) . enumerate () { if i == 0 { value . decorate (DEFAULT_LEADING_VALUE_DECOR . 0 , DEFAULT_LEADING_VALUE_DECOR . 1) ; } else { value . decorate (DEFAULT_VALUE_DECOR . 0 , DEFAULT_VALUE_DECOR . 1) ; } } array . set_trailing_comma (false) ; array . set_trailing ("") ; }
    };
}

decorate_array!()