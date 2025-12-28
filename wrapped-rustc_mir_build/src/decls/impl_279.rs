macro_rules! deps {
    () => {
        Rust2024IncompatiblePatSugg!();
    };
}

macro_rules! impl_279 {
    () => {
        deps!();
        impl Subdiagnostic for Rust2024IncompatiblePatSugg { fn add_to_diag < G : EmissionGuarantee > (self , diag : & mut Diag < '_ , G >) { for (span , def_br_mutbl) in self . default_mode_labels . into_iter () . rev () { if ! span . from_expansion () { let note_msg = "matching on a reference type with a non-reference pattern changes the default binding mode" ; let label_msg = format ! ("this matches on type `{}_`" , def_br_mutbl . ref_prefix_str ()) ; let mut label = MultiSpan :: from (span) ; label . push_span_label (span , label_msg) ; diag . span_note (label , note_msg) ; } } let applicability = if self . suggestion . iter () . all (| (span , _) | span . can_be_used_for_suggestions ()) { Applicability :: MachineApplicable } else { Applicability :: MaybeIncorrect } ; let msg = if self . suggest_eliding_modes { let plural_modes = pluralize ! (self . binding_mode_count) ; format ! ("remove the unnecessary binding modifier{plural_modes}") } else { let plural_derefs = pluralize ! (self . ref_pattern_count) ; let and_modes = if self . binding_mode_count > 0 { format ! (" and variable binding mode{}" , pluralize ! (self . binding_mode_count)) } else { String :: new () } ; format ! ("make the implied reference pattern{plural_derefs}{and_modes} explicit") } ; if ! self . suggestion . is_empty () { diag . multipart_suggestion_verbose (msg , self . suggestion , applicability) ; } } }
    };
}

impl_279!();