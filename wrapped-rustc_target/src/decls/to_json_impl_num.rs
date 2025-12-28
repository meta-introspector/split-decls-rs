macro_rules! deps {
    () => {
        ToJson!();
    };
}

macro_rules! to_json_impl_num {
    () => {
        deps!();
        macro_rules ! to_json_impl_num { ($ ($ t : ty) , +) => ($ (impl ToJson for $ t { fn to_json (& self) -> Json { Json :: Number (Number :: from (* self)) } }) +) }
    };
}

to_json_impl_num!();