macro_rules! UnitOnly {
    () => {
        pub (crate) struct UnitOnly < E > { marker : std :: marker :: PhantomData < E > , }
    };
}

UnitOnly!()