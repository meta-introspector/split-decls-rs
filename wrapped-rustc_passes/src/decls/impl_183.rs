macro_rules! deps {
    () => {
        DuplicateLangItem!();
        Duplicate!();
    };
}

macro_rules! impl_183 {
    () => {
        deps!();
        impl < G : EmissionGuarantee > Diagnostic < '_ , G > for DuplicateLangItem { # [track_caller] fn into_diag (self , dcx : DiagCtxtHandle < '_ > , level : Level) -> Diag < '_ , G > { let mut diag = Diag :: new (dcx , level , match self . duplicate { Duplicate :: Plain => fluent :: passes_duplicate_lang_item , Duplicate :: Crate => fluent :: passes_duplicate_lang_item_crate , Duplicate :: CrateDepends => fluent :: passes_duplicate_lang_item_crate_depends , } ,) ; diag . code (E0152) ; diag . arg ("lang_item_name" , self . lang_item_name) ; diag . arg ("crate_name" , self . crate_name) ; if let Some (dependency_of) = self . dependency_of { diag . arg ("dependency_of" , dependency_of) ; } diag . arg ("path" , self . path) ; if let Some (orig_crate_name) = self . orig_crate_name { diag . arg ("orig_crate_name" , orig_crate_name) ; } if let Some (orig_dependency_of) = self . orig_dependency_of { diag . arg ("orig_dependency_of" , orig_dependency_of) ; } diag . arg ("orig_path" , self . orig_path) ; if let Some (span) = self . local_span { diag . span (span) ; } if let Some (span) = self . first_defined_span { diag . span_note (span , fluent :: passes_first_defined_span) ; } else { if self . orig_dependency_of . is_none () { diag . note (fluent :: passes_first_defined_crate) ; } else { diag . note (fluent :: passes_first_defined_crate_depends) ; } if self . orig_is_local { diag . note (fluent :: passes_first_definition_local) ; } else { diag . note (fluent :: passes_first_definition_path) ; } if self . is_local { diag . note (fluent :: passes_second_definition_local) ; } else { diag . note (fluent :: passes_second_definition_path) ; } } diag } }
    };
}

impl_183!()