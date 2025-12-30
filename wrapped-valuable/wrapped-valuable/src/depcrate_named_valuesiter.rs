// Generated macro for Iter (struct)
macro_rules! Depcrate_named_valuesIter {
() => {
// Module: crate::named_values
// Provides: {"Iter"}
// Dependencies: {}
# [doc = " An iterator of name-value pairs contained by [`NamedValues`]."] # [doc = ""] # [doc = " Instances are created by the [`iter()`][NamedValues::iter] method on"] # [doc = " [`NamedValues`]. See its documentation for more."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use valuable::{NamedField, NamedValues, Value};"] # [doc = ""] # [doc = " let fields = ["] # [doc = "     NamedField::new(\"foo\"),"] # [doc = "     NamedField::new(\"bar\")"] # [doc = " ];"] # [doc = " let values = ["] # [doc = "     Value::U32(123),"] # [doc = "     Value::U32(456),"] # [doc = " ];"] # [doc = ""] # [doc = " let named_values = NamedValues::new(&fields, &values);"] # [doc = ""] # [doc = " for (field, value) in named_values.iter() {"] # [doc = "     println!(\"{:?}: {:?}\", field, value);"] # [doc = " }"] # [doc = " ```"] # [derive (Debug)] pub struct Iter < 'a , 'b > { iter : iter :: Enumerate < core :: slice :: Iter < 'b , NamedField < 'a > > > , values : & 'a [Value < 'a >] , }
};
}
