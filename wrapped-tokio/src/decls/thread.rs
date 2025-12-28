macro_rules! thread {
    () => {
        pub (crate) mod thread { pub use loom :: lazy_static :: AccessError ; pub use loom :: thread :: * ; }
    };
}

thread!();