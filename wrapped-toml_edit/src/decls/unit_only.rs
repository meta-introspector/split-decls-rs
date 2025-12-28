macro_rules! deps {
    () => {
        UnitOnly!();
    };
}

macro_rules! unit_only {
    () => {
        deps!();
        fn unit_only < T , E > (t : T) -> (T , UnitOnly < E >) { (t , UnitOnly { marker : std :: marker :: PhantomData , } ,) }
    };
}

unit_only!();