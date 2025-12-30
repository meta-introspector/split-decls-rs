// Generated macro for impl_862 (impl)
macro_rules! Depcrate_punctuatedimpl_862 {
() => {
// Module: crate::punctuated
// Provides: {"impl_862"}
// Dependencies: {}
# [cfg (feature = "extra-traits")] # [cfg_attr (docsrs , doc (cfg (feature = "extra-traits")))] impl < T , P > Hash for Punctuated < T , P > where T : Hash , P : Hash , { fn hash < H : Hasher > (& self , state : & mut H) { let Punctuated { inner , last } = self ; inner . hash (state) ; last . hash (state) ; } }
};
}
