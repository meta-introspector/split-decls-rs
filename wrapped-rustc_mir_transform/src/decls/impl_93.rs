macro_rules! deps {
    () => {
        LocalLabel!();
    };
}

macro_rules! impl_93 {
    () => {
        deps!();
        # [doc = " A custom `Subdiagnostic` implementation so that the notes are delivered in a specific order"] impl Subdiagnostic for LocalLabel < '_ > { fn add_to_diag < G : rustc_errors :: EmissionGuarantee > (self , diag : & mut rustc_errors :: Diag < '_ , G >) { diag . remove_arg ("name") ; diag . arg ("name" , self . name) ; diag . remove_arg ("is_generated_name") ; diag . arg ("is_generated_name" , self . is_generated_name) ; diag . remove_arg ("is_dropped_first_edition_2024") ; diag . arg ("is_dropped_first_edition_2024" , self . is_dropped_first_edition_2024) ; let msg = diag . eagerly_translate (crate :: fluent_generated :: mir_transform_tail_expr_local) ; diag . span_label (self . span , msg) ; for dtor in self . destructors { dtor . add_to_diag (diag) ; } let msg = diag . eagerly_translate (crate :: fluent_generated :: mir_transform_label_local_epilogue) ; diag . span_label (self . span , msg) ; } }
    };
}

impl_93!();