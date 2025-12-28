macro_rules! deps {
    () => {
        ThirPrinter!();
    };
}

macro_rules! impl_347 {
    () => {
        deps!();
        impl < 'a , 'tcx > Write for ThirPrinter < 'a , 'tcx > { fn write_str (& mut self , s : & str) -> fmt :: Result { self . fmt . push_str (s) ; Ok (()) } }
    };
}

impl_347!()