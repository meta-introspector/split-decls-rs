macro_rules! deps {
    () => {
        CachedThreadLocal!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl < T : Send + Default > CachedThreadLocal < T > { # [doc = " Returns the element for the current thread, or creates a default one if"] # [doc = " it doesn't exist."] pub fn get_or_default (& self) -> & T { self . get_or (T :: default) } }
    };
}

impl_5!()