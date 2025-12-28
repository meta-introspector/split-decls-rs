macro_rules! UnionField {
    () => {
        # [doc = " This represents a toplevel union field."] # [doc = ""] # [doc = " This is called `__BindgenUnionField` in bindgen bindings."] pub struct UnionField < T > (:: core :: marker :: PhantomData < T >) ;
    };
}

UnionField!();