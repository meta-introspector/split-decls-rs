macro_rules! deps {
    () => {
        ToDebug!();
        IntoData!();
        IntoJson!();
    };
}

macro_rules! prelude {
    () => {
        deps!();
        # [doc = " Easier access to common traits"] pub mod prelude { pub use crate :: IntoData ; # [cfg (feature = "json")] pub use crate :: IntoJson ; pub use crate :: ToDebug ; }
    };
}

prelude!()