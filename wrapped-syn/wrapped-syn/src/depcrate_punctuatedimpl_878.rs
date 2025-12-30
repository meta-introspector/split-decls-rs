// Generated macro for impl_878 (impl)
macro_rules! Depcrate_punctuatedimpl_878 {
() => {
// Module: crate::punctuated
// Provides: {"impl_878"}
// Dependencies: {}
# [cfg (feature = "extra-traits")] # [cfg_attr (docsrs , doc (cfg (feature = "extra-traits")))] impl < T , P > PartialEq for Punctuated < T , P > where T : PartialEq , P : PartialEq , { fn eq (& self , other : & Self) -> bool { let Punctuated { inner , last } = self ; * inner == other . inner && * last == other . last } }
};
}
