// Generated macro for deserialize_unit_struct (function)
macro_rules! Depcrate_dedeserialize_unit_struct {
() => {
// Module: crate::de
// Provides: {"deserialize_unit_struct"}
// Dependencies: {}
fn deserialize_unit_struct (params : & Parameters , cattrs : & attr :: Container) -> Fragment { let this_type = & params . this_type ; let this_value = & params . this_value ; let type_name = cattrs . name () . deserialize_name () ; let (de_impl_generics , de_ty_generics , ty_generics , where_clause) = split_with_de_lifetime (params) ; let delife = params . borrowed . de_lifetime () ; let expecting = format ! ("unit struct {}" , params . type_name ()) ; let expecting = cattrs . expecting () . unwrap_or (& expecting) ; quote_block ! { # [doc (hidden)] struct __Visitor # de_impl_generics # where_clause { marker : _serde :: __private :: PhantomData <# this_type # ty_generics >, lifetime : _serde :: __private :: PhantomData <&# delife () >, } # [automatically_derived] impl # de_impl_generics _serde :: de :: Visitor <# delife > for __Visitor # de_ty_generics # where_clause { type Value = # this_type # ty_generics ; fn expecting (& self , __formatter : & mut _serde :: __private :: Formatter) -> _serde :: __private :: fmt :: Result { _serde :: __private :: Formatter :: write_str (__formatter , # expecting) } # [inline] fn visit_unit < __E > (self) -> _serde :: __private :: Result < Self :: Value , __E > where __E : _serde :: de :: Error , { _serde :: __private :: Ok (# this_value) } } _serde :: Deserializer :: deserialize_unit_struct (__deserializer , # type_name , __Visitor { marker : _serde :: __private :: PhantomData ::<# this_type # ty_generics >, lifetime : _serde :: __private :: PhantomData , } ,) } }
};
}
