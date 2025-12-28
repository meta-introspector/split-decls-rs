macro_rules! write_list {
    () => {
        fn write_list < T : Display > (list : impl IntoIterator < Item = T > , f : & mut Formatter < '_ >) -> fmt :: Result { f . write_char ('[') ? ; let mut iter = list . into_iter () ; if let Some (item) = iter . next () { item . fmt (f) ? ; } for item in iter { f . write_str (", ") ? ; item . fmt (f) ? ; } f . write_char (']') }
    };
}

write_list!()