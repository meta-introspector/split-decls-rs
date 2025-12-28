macro_rules! deps {
    () => {
        Interner!();
        Ty!();
    };
}

macro_rules! VarianceDiagInfo {
    () => {
        deps!();
        # [doc = " Extra information about why we ended up with a particular variance."] # [doc = " This is only used to add more information to error messages, and"] # [doc = " has no effect on soundness. While choosing the 'wrong' `VarianceDiagInfo`"] # [doc = " may lead to confusing notes in error messages, it will never cause"] # [doc = " a miscompilation or unsoundness."] # [doc = ""] # [doc = " When in doubt, use `VarianceDiagInfo::default()`"] # [derive_where (Clone , Copy , PartialEq , Debug , Default ; I : Interner)] pub enum VarianceDiagInfo < I : Interner > { # [doc = " No additional information - this is the default."] # [doc = " We will not add any additional information to error messages."] # [derive_where (default)] None , # [doc = " We switched our variance because a generic argument occurs inside"] # [doc = " the invariant generic argument of another type."] Invariant { # [doc = " The generic type containing the generic parameter"] # [doc = " that changes the variance (e.g. `*mut T`, `MyStruct<T>`)"] ty : I :: Ty , # [doc = " The index of the generic parameter being used"] # [doc = " (e.g. `0` for `*mut T`, `1` for `MyStruct<'CovariantParam, 'InvariantParam>`)"] param_index : u32 , } , }
    };
}

VarianceDiagInfo!()