// Generated macro for impl_80 (impl)
macro_rules! Depcrate_first_passimpl_80 {
() => {
// Module: crate::first_pass
// Provides: {"impl_80"}
// Dependencies: {}
impl < 'a > FirstPassRecord < 'a > { pub fn all_superclasses < 'me > (& 'me self , interface : & str) -> impl Iterator < Item = String > + 'me { let mut set = BTreeSet :: new () ; let mut list = Vec :: new () ; self . fill_superclasses (interface , & mut set , & mut list) ; list . into_iter () } fn fill_superclasses (& self , interface : & str , set : & mut BTreeSet < & 'a str > , list : & mut Vec < String > ,) { let data = match self . interfaces . get (interface) { Some (data) => data , None => return , } ; let superclass = match & data . superclass { Some (class) => class , None => return , } ; if self . interfaces . contains_key (superclass) && set . insert (superclass) { list . push (camel_case_ident (superclass)) ; self . fill_superclasses (superclass , set , list) ; } } pub fn all_mixins < 'me > (& 'me self , interface : & str ,) -> impl Iterator < Item = & 'me MixinData < 'a > > + 'me { let mut set = Vec :: new () ; self . fill_mixins (interface , & mut set) ; set . into_iter () } fn fill_mixins < 'me > (& 'me self , mixin_name : & str , list : & mut Vec < & 'me MixinData < 'a > >) { if let Some (mixin_data) = self . mixins . get (mixin_name) { list . push (mixin_data) ; } if let Some (mixin_names) = self . includes . get (mixin_name) { for mixin_name in mixin_names { self . fill_mixins (mixin_name , list) ; } } } }
};
}
