macro_rules! write_object {
    () => {
        fn write_object < K : Display , V : Display > (object : impl IntoIterator < Item = (K , V) > , f : & mut Formatter < '_ > ,) -> fmt :: Result { f . write_char ('{') ? ; let mut iter = object . into_iter () ; if let Some ((name , value)) = iter . next () { write ! (f , "{}: {}" , name , value) ? ; } for (name , value) in iter { f . write_str (", ") ? ; write ! (f , "{}: {}" , name , value) ? ; } f . write_char ('}') }
    };
}

write_object!();