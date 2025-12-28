macro_rules! deps {
    () => {
        ExpnId!();
    };
}

macro_rules! impl_44 {
    () => {
        deps!();
        impl fmt :: Debug for ExpnId { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{:?}::{{{{expn{}}}}}" , self . krate , self . local_id . as_u32 ()) } }
    };
}

impl_44!()