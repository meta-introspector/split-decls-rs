macro_rules! Upcast {
    () => {
        # [doc = " An `Into`-like trait that takes `TyCtxt` to perform interner-specific transformations."] pub trait Upcast < I , T > { fn upcast (self , interner : I) -> T ; }
    };
}

Upcast!();