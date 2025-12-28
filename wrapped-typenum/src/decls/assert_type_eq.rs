macro_rules! deps {
    () => {
        Same!();
    };
}

macro_rules! assert_type_eq {
    () => {
        deps!();
        # [doc = " Asserts that two types are the same."] # [macro_export] macro_rules ! assert_type_eq { ($ a : ty , $ b : ty) => { const _ : core :: marker :: PhantomData <<$ a as $ crate :: Same <$ b >>:: Output > = core :: marker :: PhantomData ; } ; }
    };
}

assert_type_eq!()