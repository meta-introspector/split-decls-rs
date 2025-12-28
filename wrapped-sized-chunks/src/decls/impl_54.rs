macro_rules! deps {
    () => {
        Chunk!();
    };
}

macro_rules! impl_54 {
    () => {
        deps!();
        impl < A , const N : usize > Clone for Chunk < A , N > where A : Clone , { fn clone (& self) -> Self { let mut out = Self :: new () ; out . left = self . left ; out . right = self . left ; for index in self . left .. self . right { unsafe { Chunk :: force_write (index , (* self . ptr (index)) . clone () , & mut out) } out . right = index + 1 ; } out } }
    };
}

impl_54!();