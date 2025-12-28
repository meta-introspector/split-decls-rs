macro_rules! deps {
    () => {
        Array!();
        Item!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        # [doc = " Constructors"] # [doc = ""] # [doc = " See also `FromIterator`"] impl Array { # [doc = " Create an empty `Array`"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " let mut arr = toml_edit::Array::new();"] # [doc = " ```"] pub fn new () -> Self { Default :: default () } pub (crate) fn with_vec (values : Vec < Item >) -> Self { Self { values , .. Default :: default () } } }
    };
}

impl_4!();