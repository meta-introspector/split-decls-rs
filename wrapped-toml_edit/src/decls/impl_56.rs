macro_rules! deps {
    () => {
        ValueRepr!();
        Repr!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        impl ValueRepr for String { fn to_repr (& self) -> Repr { let output = toml_writer :: TomlStringBuilder :: new (self . as_str ()) . as_default () . to_toml_value () ; Repr :: new_unchecked (output) } }
    };
}

impl_56!()