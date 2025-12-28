macro_rules! deps {
    () => {
        Buffer!();
    };
}

macro_rules! impl_280 {
    () => {
        deps!();
        impl core :: fmt :: Display for Buffer { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { let mut tables = self . tables . iter () . filter_map (| t | t . as_ref ()) . filter (| t | required_table (t)) ; if let Some (table) = tables . next () { table . fmt (f) ? ; } for table in tables { f . newline () ? ; table . fmt (f) ? ; } Ok (()) } }
    };
}

impl_280!();