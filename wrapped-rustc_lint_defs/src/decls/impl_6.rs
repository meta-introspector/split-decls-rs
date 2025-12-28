macro_rules! deps {
    () => {
        LintExpectationId!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl < HCX : HashStableContext > HashStable < HCX > for LintExpectationId { # [inline] fn hash_stable (& self , hcx : & mut HCX , hasher : & mut StableHasher) { match self { LintExpectationId :: Stable { hir_id , attr_index , lint_index : Some (lint_index) } => { hir_id . hash_stable (hcx , hasher) ; attr_index . hash_stable (hcx , hasher) ; lint_index . hash_stable (hcx , hasher) ; } _ => { unreachable ! ("HashStable should only be called for filled and stable `LintExpectationId`") } } } }
    };
}

impl_6!()