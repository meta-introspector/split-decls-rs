macro_rules! deps {
    () => {
        Downcast!();
    };
}

macro_rules! IntoBox {
    () => {
        deps!();
        # [doc = " A trait for the conversion of an object into a boxed trait object."] pub trait IntoBox < A : ? Sized + Downcast > : Any { # [doc = " Convert self into the appropriate boxed form."] fn into_box (self) -> Box < A > ; }
    };
}

IntoBox!();