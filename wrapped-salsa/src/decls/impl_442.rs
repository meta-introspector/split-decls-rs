macro_rules! deps {
    () => {
        MemoIngredientIndex!();
    };
}

macro_rules! impl_442 {
    () => {
        deps!();
        impl MemoIngredientIndex { pub (crate) fn from_usize (u : usize) -> Self { assert ! (u <= u32 :: MAX as usize) ; MemoIngredientIndex (u as u32) } # [inline] pub (crate) fn as_usize (self) -> usize { self . 0 as usize } }
    };
}

impl_442!()