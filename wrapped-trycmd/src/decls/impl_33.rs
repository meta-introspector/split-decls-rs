macro_rules! deps {
    () => {
        Bin!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        impl < P , E > From < Result < P , E > > for Bin where P : Into < Bin > , E : std :: fmt :: Display , { fn from (other : Result < P , E >) -> Self { match other { Ok (path) => path . into () , Err (err) => { let err = crate :: Error :: new (err . to_string ()) ; Bin :: Error (err) } } } }
    };
}

impl_33!()