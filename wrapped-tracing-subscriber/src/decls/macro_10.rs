macro_rules! macro_10 {
    () => {
        feature ! { #! [all (feature = "fmt" , feature = "std")] pub mod fmt ; pub use fmt :: fmt ; pub use fmt :: Subscriber as FmtSubscriber ; }
    };
}

macro_10!()