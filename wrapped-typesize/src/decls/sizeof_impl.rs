macro_rules! deps {
    () => {
        TypeSize!();
    };
}

macro_rules! sizeof_impl {
    () => {
        deps!();
        # [doc = " Implements [`TypeSize`] for multiple types based on the return value of [`core::mem::size_of`]."] # [macro_export] macro_rules ! sizeof_impl { ($ ($ ty : ty) ,*) => { $ (impl TypeSize for $ ty { }) * } ; }
    };
}

sizeof_impl!()