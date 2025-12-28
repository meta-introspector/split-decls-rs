macro_rules! write_binary {
    () => {
        fn write_binary (bytes : & [u8] , f : & mut Formatter < '_ >) -> fmt :: Result { f . write_char ('[') ? ; let mut iter = bytes . iter () . copied () ; if let Some (value) = iter . next () { value . fmt (f) ? ; } for value in iter { f . write_str (", ") ? ; value . fmt (f) ? ; } f . write_char (']') }
    };
}

write_binary!()