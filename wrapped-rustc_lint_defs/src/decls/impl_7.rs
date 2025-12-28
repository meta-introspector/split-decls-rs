macro_rules! deps {
    () => {
        LintExpectationId!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl < HCX : HashStableContext > ToStableHashKey < HCX > for LintExpectationId { type KeyType = (DefPathHash , ItemLocalId , u16 , u16) ; # [inline] fn to_stable_hash_key (& self , hcx : & HCX) -> Self :: KeyType { match self { LintExpectationId :: Stable { hir_id , attr_index , lint_index : Some (lint_index) } => { let (def_path_hash , lint_idx) = hir_id . to_stable_hash_key (hcx) ; (def_path_hash , lint_idx , * attr_index , * lint_index) } _ => { unreachable ! ("HashStable should only be called for a filled `LintExpectationId`") } } } }
    };
}

impl_7!()