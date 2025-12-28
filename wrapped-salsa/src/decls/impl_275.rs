macro_rules! deps {
    () => {
        Running!();
    };
}

macro_rules! impl_275 {
    () => {
        deps!();
        impl std :: fmt :: Debug for Running < '_ > { fn fmt (& self , fmt : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { fmt . debug_struct ("Running") . field ("database_key" , & self . 0 . database_key) . field ("other_id" , & self . 0 . other_id) . field ("thread_id" , & self . 0 . thread_id) . finish () } }
    };
}

impl_275!()