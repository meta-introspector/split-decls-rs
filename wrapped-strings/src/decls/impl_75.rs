macro_rules! deps {
    () => {
        HStringBuilder!();
        Decode!();
    };
}

macro_rules! impl_75 {
    () => {
        deps!();
        impl core :: fmt :: Debug for HStringBuilder { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { write ! (f , "\"{}\"" , Decode (|| core :: char :: decode_utf16 (self . iter () . cloned ()))) } }
    };
}

impl_75!()