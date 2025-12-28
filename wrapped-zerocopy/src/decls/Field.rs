macro_rules! Field {
    () => {
        # [doc = " Projects the type of the field at `Index` in `Self`."] # [doc = ""] # [doc = " The `Index` parameter is any sort of handle that identifies the field; its"] # [doc = " definition is the obligation of the implementer."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Unsafe code may assume that this accurately reflects the definition of"] # [doc = " `Self`."] pub unsafe trait Field < Index > { # [doc = " The type of the field at `Index`."] type Type : ? Sized ; }
    };
}

Field!()