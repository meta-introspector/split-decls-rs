macro_rules! assert_type {
    () => {
        # [doc = " Asserts that a type is `True`, aka `B1`."] # [macro_export] macro_rules ! assert_type { ($ a : ty) => { const _ : core :: marker :: PhantomData <<$ a as $ crate :: Same < True >>:: Output > = core :: marker :: PhantomData ; } ; }
    };
}

assert_type!()