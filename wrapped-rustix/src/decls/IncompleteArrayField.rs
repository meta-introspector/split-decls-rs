macro_rules! IncompleteArrayField {
    () => {
        # [doc = " This represents an incomplete array field at the end of a struct."] # [doc = ""] # [doc = " This is called `__IncompleteArrayField` in bindgen bindings."] # [repr (C)] # [derive (Default)] pub struct IncompleteArrayField < T > (:: core :: marker :: PhantomData < T > , [T ; 0]) ;
    };
}

IncompleteArrayField!()