macro_rules! option_as_ptr {
    () => {
        # [doc = " Convert an `Option<&T>` into a possibly-null `*const T`."] # [inline] pub (crate) const fn option_as_ptr < T > (t : Option < & T >) -> * const T { match t { Some (t) => t , None => null () , } }
    };
}

option_as_ptr!();