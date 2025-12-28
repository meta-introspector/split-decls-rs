macro_rules! deps {
    () => {
        PaddingCheck!();
    };
}

macro_rules! impl_106 {
    () => {
        deps!();
        impl PaddingCheck { # [doc = " Returns the idents of the trait to use and the macro to call in order to"] # [doc = " validate that a type passes the relevant padding check."] fn validator_trait_and_macro_idents (& self) -> (Ident , Ident) { let (trt , mcro) = match self { PaddingCheck :: Struct => ("PaddingFree" , "struct_padding") , PaddingCheck :: ReprCStruct => ("DynamicPaddingFree" , "repr_c_struct_has_padding") , PaddingCheck :: Union => ("PaddingFree" , "union_padding") , PaddingCheck :: Enum { .. } => ("PaddingFree" , "enum_padding") , } ; let trt = Ident :: new (trt , Span :: call_site ()) ; let mcro = Ident :: new (mcro , Span :: call_site ()) ; (trt , mcro) } # [doc = " Sometimes performing the padding check requires some additional"] # [doc = " \"context\" code. For enums, this is the definition of the tag enum."] fn validator_macro_context (& self) -> Option < & TokenStream > { match self { PaddingCheck :: Struct | PaddingCheck :: ReprCStruct | PaddingCheck :: Union => None , PaddingCheck :: Enum { tag_type_definition } => Some (tag_type_definition) , } } }
    };
}

impl_106!()