macro_rules! deps {
    () => {
        MaybeItemFn!();
        MaybeItemFnRef!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl MaybeItemFn { fn as_ref (& self) -> MaybeItemFnRef < '_ , TokenStream > { MaybeItemFnRef { outer_attrs : & self . outer_attrs , inner_attrs : & self . inner_attrs , vis : & self . vis , sig : & self . sig , brace_token : & self . brace_token , block : & self . block , } } }
    };
}

impl_7!()