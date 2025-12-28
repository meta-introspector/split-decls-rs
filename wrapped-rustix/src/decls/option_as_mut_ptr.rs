macro_rules! option_as_mut_ptr {
    () => {
        # [doc = " Convert an `Option<&mut T>` into a possibly-null `*mut T`."] # [inline] pub (crate) fn option_as_mut_ptr < T > (t : Option < & mut T >) -> * mut T { match t { Some (t) => t , None => null_mut () , } }
    };
}

option_as_mut_ptr!()