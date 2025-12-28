macro_rules! UnitOnly {
    () => {
        pub (crate) struct UnitOnly < E > { marker : core :: marker :: PhantomData < E > , }
    };
}

UnitOnly!()