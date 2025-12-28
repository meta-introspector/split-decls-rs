macro_rules! deps {
    () => {
        Database!();
    };
}

macro_rules! RawDatabase {
    () => {
        deps!();
        # [derive (Copy , Clone)] pub struct RawDatabase < 'db > { pub (crate) ptr : NonNull < () > , _marker : std :: marker :: PhantomData < & 'db dyn Database > , }
    };
}

RawDatabase!()