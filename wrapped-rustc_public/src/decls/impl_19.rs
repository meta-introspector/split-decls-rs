macro_rules! deps {
    () => {
        LayoutShape!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl LayoutShape { # [doc = " Returns `true` if the layout corresponds to an unsized type."] # [inline] pub fn is_unsized (& self) -> bool { self . abi . is_unsized () } # [inline] pub fn is_sized (& self) -> bool { ! self . abi . is_unsized () } # [doc = " Returns `true` if the type is sized and a 1-ZST (meaning it has size 0 and alignment 1)."] pub fn is_1zst (& self) -> bool { self . is_sized () && self . size . bits () == 0 && self . abi_align == 1 } }
    };
}

impl_19!();