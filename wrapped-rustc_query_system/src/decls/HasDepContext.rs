macro_rules! deps {
    () => {
        DepContext!();
        Deps!();
    };
}

macro_rules! HasDepContext {
    () => {
        deps!();
        pub trait HasDepContext : Copy { type Deps : self :: Deps ; type DepContext : self :: DepContext < Deps = Self :: Deps > ; fn dep_context (& self) -> & Self :: DepContext ; }
    };
}

HasDepContext!();