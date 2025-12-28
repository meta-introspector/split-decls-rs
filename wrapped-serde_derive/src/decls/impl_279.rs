macro_rules! deps {
    () => {
        TupleTrait!();
    };
}

macro_rules! impl_279 {
    () => {
        deps!();
        impl TupleTrait { fn serialize_element (& self , span : Span) -> TokenStream { match * self { TupleTrait :: SerializeTuple => { quote_spanned ! (span => _serde :: ser :: SerializeTuple :: serialize_element) } TupleTrait :: SerializeTupleStruct => { quote_spanned ! (span => _serde :: ser :: SerializeTupleStruct :: serialize_field) } TupleTrait :: SerializeTupleVariant => { quote_spanned ! (span => _serde :: ser :: SerializeTupleVariant :: serialize_field) } } } }
    };
}

impl_279!()