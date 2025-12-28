macro_rules! impl_50 {
    () => {
        # [cfg (feature = "std")] # [cfg_attr (docsrs , doc (cfg (feature = "std")))] impl < T > WithDispatch < T > { # [doc = " Borrows the [`Dispatch`] that is entered when this type is polled."] pub fn dispatcher (& self) -> & Dispatch { & self . dispatcher } # [doc = " Borrows the wrapped type."] pub fn inner (& self) -> & T { & self . inner } # [doc = " Mutably borrows the wrapped type."] pub fn inner_mut (& mut self) -> & mut T { & mut self . inner } # [doc = " Get a pinned reference to the wrapped type."] pub fn inner_pin_ref (self : Pin < & Self >) -> Pin < & T > { self . project_ref () . inner } # [doc = " Get a pinned mutable reference to the wrapped type."] pub fn inner_pin_mut (self : Pin < & mut Self >) -> Pin < & mut T > { self . project () . inner } # [doc = " Consumes the `Instrumented`, returning the wrapped type."] # [doc = ""] # [doc = " Note that this drops the span."] pub fn into_inner (self) -> T { self . inner } }
    };
}

impl_50!()