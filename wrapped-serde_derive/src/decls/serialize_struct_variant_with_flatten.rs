macro_rules! deps {
    () => {
        StructVariant!();
        Name!();
        StructTrait!();
        Parameters!();
        Fragment!();
        Field!();
    };
}

macro_rules! serialize_struct_variant_with_flatten {
    () => {
        deps!();
        fn serialize_struct_variant_with_flatten (context : StructVariant , params : & Parameters , fields : & [Field] , name : & Name ,) -> Fragment { let struct_trait = StructTrait :: SerializeMap ; let serialize_fields = serialize_struct_visitor (fields , params , true , & struct_trait) ; let mut serialized_fields = fields . iter () . filter (| & field | ! field . attrs . skip_serializing ()) . peekable () ; let let_mut = mut_if (serialized_fields . peek () . is_some ()) ; match context { StructVariant :: ExternallyTagged { variant_index , variant_name , } => { let this_type = & params . this_type ; let fields_ty = fields . iter () . map (| f | & f . ty) ; let members = & fields . iter () . map (| f | & f . member) . collect :: < Vec < _ > > () ; let (_ , ty_generics , where_clause) = params . generics . split_for_impl () ; let wrapper_generics = bound :: with_lifetime_bound (& params . generics , "'__a") ; let (wrapper_impl_generics , wrapper_ty_generics , _) = wrapper_generics . split_for_impl () ; quote_block ! { # [doc (hidden)] struct __EnumFlatten # wrapper_generics # where_clause { data : (# (&'__a # fields_ty ,) *) , phantom : _serde ::# private :: PhantomData <# this_type # ty_generics >, } # [automatically_derived] impl # wrapper_impl_generics _serde :: Serialize for __EnumFlatten # wrapper_ty_generics # where_clause { fn serialize < __S > (& self , __serializer : __S) -> _serde ::# private :: Result < __S :: Ok , __S :: Error > where __S : _serde :: Serializer , { let (# (# members ,) *) = self . data ; let # let_mut __serde_state = _serde :: Serializer :: serialize_map (__serializer , _serde ::# private :: None) ?; # (# serialize_fields) * _serde :: ser :: SerializeMap :: end (__serde_state) } } _serde :: Serializer :: serialize_newtype_variant (__serializer , # name , # variant_index , # variant_name , & __EnumFlatten { data : (# (# members ,) *) , phantom : _serde ::# private :: PhantomData ::<# this_type # ty_generics >, }) } } StructVariant :: InternallyTagged { tag , variant_name } => { quote_block ! { let # let_mut __serde_state = _serde :: Serializer :: serialize_map (__serializer , _serde ::# private :: None) ?; _serde :: ser :: SerializeMap :: serialize_entry (& mut __serde_state , # tag , # variant_name ,) ?; # (# serialize_fields) * _serde :: ser :: SerializeMap :: end (__serde_state) } } StructVariant :: Untagged => { quote_block ! { let # let_mut __serde_state = _serde :: Serializer :: serialize_map (__serializer , _serde ::# private :: None) ?; # (# serialize_fields) * _serde :: ser :: SerializeMap :: end (__serde_state) } } } }
    };
}

serialize_struct_variant_with_flatten!()