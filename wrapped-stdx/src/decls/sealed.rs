macro_rules! sealed {
    () => {
        mod sealed { pub trait Sealed { const VALUE : Self ; } }
    };
}

sealed!()