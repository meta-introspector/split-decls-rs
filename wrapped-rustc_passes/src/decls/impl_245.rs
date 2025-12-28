macro_rules! deps {
    () => {
        HirIdValidator!();
    };
}

macro_rules! impl_245 {
    () => {
        deps!();
        impl < 'a , 'hir > HirIdValidator < 'a , 'hir > { fn new_visitor (& self , tcx : TyCtxt < 'hir >) -> HirIdValidator < 'a , 'hir > { HirIdValidator { tcx , owner : None , hir_ids_seen : Default :: default () , errors : self . errors } } # [cold] # [inline (never)] fn error (& self , f : impl FnOnce () -> String) { self . errors . lock () . push (f ()) ; } fn check < F : FnOnce (& mut HirIdValidator < 'a , 'hir >) > (& mut self , owner : hir :: OwnerId , walk : F) { assert ! (self . owner . is_none ()) ; self . owner = Some (owner) ; walk (self) ; if owner == hir :: CRATE_OWNER_ID { return ; } let max = self . hir_ids_seen . iter () . map (| local_id | local_id . as_usize ()) . max () . expect ("owning item has no entry") ; if max != self . hir_ids_seen . len () - 1 { let pretty_owner = self . tcx . hir_def_path (owner . def_id) . to_string_no_crate_verbose () ; let missing_items : Vec < _ > = (0 ..= max as u32) . map (| i | ItemLocalId :: from_u32 (i)) . filter (| & local_id | ! self . hir_ids_seen . contains (local_id)) . map (| local_id | self . tcx . hir_id_to_string (HirId { owner , local_id })) . collect () ; let seen_items : Vec < _ > = self . hir_ids_seen . iter () . map (| local_id | self . tcx . hir_id_to_string (HirId { owner , local_id })) . collect () ; self . error (| | { format ! ("ItemLocalIds not assigned densely in {pretty_owner}. \
            Max ItemLocalId = {max}, missing IDs = {missing_items:#?}; seen IDs = {seen_items:#?}") }) ; } } fn check_nested_id (& mut self , id : LocalDefId) { let Some (owner) = self . owner else { return } ; let def_parent = self . tcx . local_parent (id) ; let def_parent_hir_id = self . tcx . local_def_id_to_hir_id (def_parent) ; if def_parent_hir_id . owner != owner { self . error (| | { format ! ("inconsistent Def parent at `{:?}` for `{:?}`:\nexpected={:?}\nfound={:?}" , self . tcx . def_span (id) , id , owner , def_parent_hir_id) }) ; } } }
    };
}

impl_245!();