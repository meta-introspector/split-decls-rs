macro_rules! core_reexport {
    () => {
        pub mod core_reexport { pub use core :: * ; pub mod mem { pub use core :: mem :: * ; } }
    };
}

core_reexport!();