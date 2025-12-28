macro_rules! deps {
    () => {
        FlatSet!();
        HasBottom!();
    };
}

macro_rules! impl_85 {
    () => {
        deps!();
        impl < T > HasBottom for FlatSet < T > { const BOTTOM : Self = Self :: Bottom ; fn is_bottom (& self) -> bool { matches ! (self , Self :: Bottom) } }
    };
}

impl_85!()