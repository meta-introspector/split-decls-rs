macro_rules! quick_check {
    () => {
        pub mod quick_check { pub use crate :: quick_check :: * ; }
    };
}

quick_check!();