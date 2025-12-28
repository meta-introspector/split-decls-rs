macro_rules! deps {
    () => {
        ParseStream!();
    };
}

macro_rules! ParseNestedMeta {
    () => {
        deps!();
        # [doc = " Context for parsing a single property in the conventional syntax for"] # [doc = " structured attributes."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " Refer to usage examples on the following two entry-points:"] # [doc = ""] # [doc = " - [`Attribute::parse_nested_meta`] if you have an entire `Attribute` to"] # [doc = "   parse. Always use this if possible. Generally this is able to produce"] # [doc = "   better error messages because `Attribute` holds span information for all"] # [doc = "   of the delimiters therein."] # [doc = ""] # [doc = " - [`syn::meta::parser`] if you are implementing a `proc_macro_attribute`"] # [doc = "   macro and parsing the arguments to the attribute macro, i.e. the ones"] # [doc = "   written in the same attribute that dispatched the macro invocation. Rustc"] # [doc = "   does not pass span information for the surrounding delimiters into the"] # [doc = "   attribute macro invocation in this situation, so error messages might be"] # [doc = "   less precise."] # [doc = ""] # [doc = " [`Attribute::parse_nested_meta`]: crate::Attribute::parse_nested_meta"] # [doc = " [`syn::meta::parser`]: crate::meta::parser"] # [non_exhaustive] pub struct ParseNestedMeta < 'a > { pub path : Path , pub input : ParseStream < 'a > , }
    };
}

ParseNestedMeta!();