macro_rules! deps {
    () => {
        Sealed!();
    };
}

macro_rules! Float {
    () => {
        deps!();
        # [doc = " A floating point number, f32 or f64, that can be written into a"] # [doc = " [`ryu::Buffer`][Buffer]."] # [doc = ""] # [doc = " This trait is sealed and cannot be implemented for types outside of the"] # [doc = " `ryu` crate."] pub trait Float : Sealed { }
    };
}

Float!()