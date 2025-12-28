macro_rules! macro_77 {
    () => {
        feature ! { #! [any (feature = "std" , feature = "alloc")] pub mod targets ; pub use self :: targets :: Targets ; mod directive ; pub use self :: directive :: ParseError ; }
    };
}

macro_77!()