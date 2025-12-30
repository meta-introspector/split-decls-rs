// Generated macro for impl_50 (impl)
macro_rules! Depcrate_first_passimpl_50 {
() => {
// Module: crate::first_pass
// Provides: {"impl_50"}
// Dependencies: {}
impl < 'src > FirstPass < 'src , ApiStability > for weedle :: Definition < 'src > { fn first_pass (& 'src self , record : & mut FirstPassRecord < 'src > , stability : ApiStability ,) -> Result < () > { use weedle :: Definition :: * ; match self { Dictionary (dictionary) => dictionary . first_pass (record , stability) , PartialDictionary (dictionary) => dictionary . first_pass (record , stability) , Enum (enum_) => enum_ . first_pass (record , stability) , IncludesStatement (includes) => includes . first_pass (record , ()) , Interface (interface) => interface . first_pass (record , stability) , PartialInterface (interface) => interface . first_pass (record , stability) , InterfaceMixin (mixin) => mixin . first_pass (record , stability) , PartialInterfaceMixin (mixin) => mixin . first_pass (record , stability) , Namespace (namespace) => namespace . first_pass (record , stability) , PartialNamespace (namespace) => namespace . first_pass (record , stability) , Typedef (typedef) => typedef . first_pass (record , ()) , Callback (callback) => callback . first_pass (record , ()) , CallbackInterface (iface) => iface . first_pass (record , ()) , Implements (_) => Ok (()) , } } }
};
}
