macro_rules! deps {
    () => {
        Table!();
        Item!();
        ArrayOfTables!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl FromIterator < Table > for ArrayOfTables { fn from_iter < I > (iter : I) -> Self where I : IntoIterator < Item = Table > , { let v = iter . into_iter () . map (Item :: Table) ; Self { values : v . collect () , span : None , } } }
    };
}

impl_22!();