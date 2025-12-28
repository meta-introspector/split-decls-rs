macro_rules! IgnoredDerivedImpls {
    () => {
        # [derive (Subdiagnostic)] # [note (passes_ignored_derived_impls)] pub (crate) struct IgnoredDerivedImpls { pub name : Symbol , pub trait_list : DiagSymbolList , pub trait_list_len : usize , }
    };
}

IgnoredDerivedImpls!();