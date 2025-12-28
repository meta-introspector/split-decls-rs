macro_rules! Palette {
    () => {
        # [derive (Copy , Clone , Debug , Default)] pub struct Palette { pub (crate) info : Style , pub (crate) warn : Style , pub (crate) error : Style , pub (crate) hint : Style , pub (crate) expected : Style , pub (crate) actual : Style , }
    };
}

Palette!();