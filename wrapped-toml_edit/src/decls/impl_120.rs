macro_rules! deps {
    () => {
        Table!();
        Item!();
        ArrayOfTables!();
        Value!();
    };
}

macro_rules! impl_120 {
    () => {
        deps!();
        # [cfg (feature = "display")] impl std :: fmt :: Display for Item { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { match & self { Self :: None => Ok (()) , Self :: Value (v) => v . fmt (f) , Self :: Table (v) => v . fmt (f) , Self :: ArrayOfTables (v) => v . fmt (f) , } } }
    };
}

impl_120!()