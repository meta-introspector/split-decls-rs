macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! sym {
    () => {
        deps!();
        pub mod sym { use super :: Symbol ; pub const DERIVE : Symbol = Symbol ; pub const DEBUG : Symbol = Symbol ; }
    };
}

sym!()