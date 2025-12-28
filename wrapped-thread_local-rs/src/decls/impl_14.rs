macro_rules! deps {
    () => {
        ThreadLocal!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl < T : Send + Default > ThreadLocal < T > { # [doc = " Returns the element for the current thread, or creates a default one if"] # [doc = " it doesn't exist."] pub fn get_or_default (& self) -> & T { self . get_or (Default :: default) } }
    };
}

impl_14!()