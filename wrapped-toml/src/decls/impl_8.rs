macro_rules! deps {
    () => {
        Map!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl < K , V > Map < K , V > where K : Ord , { pub (crate) fn is_dotted (& self) -> bool { self . dotted } pub (crate) fn is_implicit (& self) -> bool { self . implicit } pub (crate) fn is_inline (& self) -> bool { self . inline } pub (crate) fn set_implicit (& mut self , yes : bool) { self . implicit = yes ; } pub (crate) fn set_dotted (& mut self , yes : bool) { self . dotted = yes ; } pub (crate) fn set_inline (& mut self , yes : bool) { self . inline = yes ; } }
    };
}

impl_8!()