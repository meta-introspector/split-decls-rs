// Generated macro for choose_fold (function)
macro_rules! Depcrate_case_foldingchoose_fold {
() => {
// Module: crate::case_folding
// Provides: {"choose_fold"}
// Dependencies: {}
# [doc = " Given a sequence of case fold mappings, choose exactly one mapping based"] # [doc = " on the mapping's status. If `full` is true, then full case mappings are"] # [doc = " selected, otherwise simple case mappings are selected. If there are"] # [doc = " multiple valid choices, then an error is returned."] fn choose_fold (case_folds : & [CaseFold] , full : bool ,) -> Result < Option < & CaseFold > > { let mut choice = None ; for case_fold in case_folds { if (full && case_fold . status == CaseStatus :: Full) || (! full && case_fold . status == CaseStatus :: Simple) || case_fold . status == CaseStatus :: Common { if choice . is_some () { return err ! ("found multiple matches from: {:?}" , case_folds) ; } choice = Some (case_fold) ; } } Ok (choice) }
};
}
