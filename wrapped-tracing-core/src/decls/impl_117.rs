macro_rules! deps {
    () => {
        HexBytes!();
    };
}

macro_rules! impl_117 {
    () => {
        deps!();
        impl fmt :: Debug for HexBytes < '_ > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_char ('[') ? ; let mut bytes = self . 0 . iter () ; if let Some (byte) = bytes . next () { f . write_fmt (format_args ! ("{byte:02x}")) ? ; } for byte in bytes { f . write_fmt (format_args ! (" {byte:02x}")) ? ; } f . write_char (']') } }
    };
}

impl_117!();