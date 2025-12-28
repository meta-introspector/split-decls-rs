macro_rules! deps {
    () => {
        Datetime!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        # [cfg (feature = "alloc")] impl fmt :: Display for Datetime { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if let Some (ref date) = self . date { write ! (f , "{date}") ? ; } if let Some (ref time) = self . time { if self . date . is_some () { write ! (f , "T") ? ; } write ! (f , "{time}") ? ; } if let Some (ref offset) = self . offset { write ! (f , "{offset}") ? ; } Ok (()) } }
    };
}

impl_13!();