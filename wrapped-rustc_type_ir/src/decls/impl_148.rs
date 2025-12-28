macro_rules! deps {
    () => {
        CandidateHeadUsages!();
        HeadUsages!();
    };
}

macro_rules! impl_148 {
    () => {
        deps!();
        impl CandidateHeadUsages { pub fn merge_usages (& mut self , other : CandidateHeadUsages) { if let Some (other_usages) = other . usages { if let Some (ref mut self_usages) = self . usages { # [allow (rustc :: potential_query_instability)] for (head_index , head) in other_usages . into_iter () { let HeadUsages { inductive , unknown , coinductive , forced_ambiguity } = head ; let self_usages = self_usages . entry (head_index) . or_default () ; self_usages . inductive += inductive ; self_usages . unknown += unknown ; self_usages . coinductive += coinductive ; self_usages . forced_ambiguity += forced_ambiguity ; } } else { self . usages = Some (other_usages) ; } } } }
    };
}

impl_148!()