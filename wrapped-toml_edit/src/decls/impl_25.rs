macro_rules! deps {
    () => {
        ArrayOfTables!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        # [cfg (feature = "display")] impl std :: fmt :: Display for ArrayOfTables { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { self . clone () . into_array () . fmt (f) } }
    };
}

impl_25!();