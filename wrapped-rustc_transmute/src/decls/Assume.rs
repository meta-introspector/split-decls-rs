macro_rules! Assume {
    () => {
        # [derive (Copy , Clone , Debug , Default)] pub struct Assume { pub alignment : bool , pub lifetimes : bool , pub safety : bool , pub validity : bool , }
    };
}

Assume!();