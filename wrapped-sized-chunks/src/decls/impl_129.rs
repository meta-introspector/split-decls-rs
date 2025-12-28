macro_rules! deps {
    () => {
        RawIndex!();
    };
}

macro_rules! impl_129 {
    () => {
        deps!();
        impl < const N : usize > RawIndex < N > { # [inline] # [must_use] pub (crate) fn to_usize (self) -> usize { self . 0 } # [doc = " Increments the index and returns a copy of the index /before/ incrementing."] # [inline] # [must_use] pub (crate) fn inc (& mut self) -> Self { let old = * self ; self . 0 = if self . 0 == N - 1 { 0 } else { self . 0 + 1 } ; old } # [doc = " Decrements the index and returns a copy of the new value."] # [inline] # [must_use] pub (crate) fn dec (& mut self) -> Self { self . 0 = if self . 0 == 0 { N - 1 } else { self . 0 - 1 } ; * self } }
    };
}

impl_129!()