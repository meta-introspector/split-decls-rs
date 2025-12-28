macro_rules! deps {
    () => {
        Bom!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl Default for Bom { # [doc = " Returns the default/empty BOM type, `Bom::Null`."] fn default () -> Self { Bom :: Null } }
    };
}

impl_15!()