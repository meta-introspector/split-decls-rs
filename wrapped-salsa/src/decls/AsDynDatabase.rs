macro_rules! deps {
    () => {
        Database!();
    };
}

macro_rules! AsDynDatabase {
    () => {
        deps!();
        # [doc = " Upcast to a `dyn Database`."] # [doc = ""] # [doc = " Only required because upcasting does not work for unsized generic parameters."] pub trait AsDynDatabase { fn as_dyn_database (& self) -> & dyn Database ; }
    };
}

AsDynDatabase!();