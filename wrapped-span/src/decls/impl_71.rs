macro_rules! deps {
    () => {
        RealSpanMap!();
    };
}

macro_rules! impl_71 {
    () => {
        deps!();
        impl fmt :: Display for RealSpanMap { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { writeln ! (f , "RealSpanMap({:?}):" , self . file_id) ? ; for span in self . pairs . iter () { writeln ! (f , "{}: {:#?}" , u32 :: from (span . 0) , span . 1) ? ; } Ok (()) } }
    };
}

impl_71!()