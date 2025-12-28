macro_rules! deps {
    () => {
        Configuration!();
        ValueShared!();
        Durability!();
    };
}

macro_rules! impl_185 {
    () => {
        deps!();
        impl ValueShared { # [doc = " Returns `true` if this value slot can be reused when interning, and should be added to the LRU."] fn is_reusable < C : Configuration > (& self) -> bool { if C :: REVISIONS == IMMORTAL { return false ; } self . durability == Durability :: LOW } }
    };
}

impl_185!();