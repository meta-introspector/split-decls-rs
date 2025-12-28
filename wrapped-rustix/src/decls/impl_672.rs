macro_rules! deps {
    () => {
        InlinedName!();
    };
}

macro_rules! impl_672 {
    () => {
        deps!();
        impl AsRef < [u8] > for InlinedName { fn as_ref (& self) -> & [u8] { & self . name [.. self . len] } }
    };
}

impl_672!()