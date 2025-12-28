macro_rules! impl_20 {
    () => {
        impl fmt :: Debug for XAttrs { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { struct AsList < 'a > (& 'a XAttrs) ; impl < 'a > fmt :: Debug for AsList < 'a > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . 0 . clone ()) . finish () } } f . debug_tuple ("XAttrs") . field (& AsList (self)) . finish () } }
    };
}

impl_20!()