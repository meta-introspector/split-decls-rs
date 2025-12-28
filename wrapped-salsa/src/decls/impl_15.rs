macro_rules! deps {
    () => {
        QueryStack!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl std :: fmt :: Debug for QueryStack { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { if f . alternate () { f . debug_list () . entries (self . stack . iter () . map (| q | q . database_key_index)) . finish () } else { f . debug_struct ("QueryStack") . field ("stack" , & self . stack) . field ("len" , & self . len) . finish () } } }
    };
}

impl_15!();