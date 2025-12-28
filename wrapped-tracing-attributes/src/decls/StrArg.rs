macro_rules! deps {
    () => {
        LitStrOrIdent!();
    };
}

macro_rules! StrArg {
    () => {
        deps!();
        struct StrArg < T > { value : LitStrOrIdent , _p : std :: marker :: PhantomData < T > , }
    };
}

StrArg!()