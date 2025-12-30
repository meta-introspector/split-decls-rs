// Generated macro for deserialize_newtype_struct (function)
macro_rules! Depcrate_dedeserialize_newtype_struct {
() => {
// Module: crate::de
// Provides: {"deserialize_newtype_struct"}
// Dependencies: {}
fn deserialize_newtype_struct (type_path : & TokenStream , params : & Parameters , field : & Field ,) -> TokenStream { let delife = params . borrowed . de_lifetime () ; let field_ty = field . ty ; let deserializer_var = quote ! (__e) ; let value = match field . attrs . deserialize_with () { None => { let span = field . original . span () ; let func = quote_spanned ! (span => <# field_ty as _serde :: Deserialize >:: deserialize) ; quote ! { # func (# deserializer_var) ? } } Some (path) => { quote_spanned ! { path . span () => # path (# deserializer_var) ? } } } ; let mut result = quote ! (# type_path (__field0)) ; if params . has_getter { let this_type = & params . this_type ; let (_ , ty_generics , _) = params . generics . split_for_impl () ; result = quote ! { _serde :: __private :: Into ::<# this_type # ty_generics >:: into (# result) } ; } quote ! { # [inline] fn visit_newtype_struct < __E > (self , # deserializer_var : __E) -> _serde :: __private :: Result < Self :: Value , __E :: Error > where __E : _serde :: Deserializer <# delife >, { let __field0 : # field_ty = # value ; _serde :: __private :: Ok (# result) } } }
};
}
