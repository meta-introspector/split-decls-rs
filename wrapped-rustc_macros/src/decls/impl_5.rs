macro_rules! deps {
    () => {
        DiagnosticDeriveKind!();
        DiagnosticDeriveError!();
        DiagnosticDerive!();
        Mismatch!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl < 'a > DiagnosticDerive < 'a > { pub (crate) fn new (structure : Structure < 'a >) -> Self { Self { structure } } pub (crate) fn into_tokens (self) -> TokenStream { let DiagnosticDerive { mut structure } = self ; let kind = DiagnosticDeriveKind :: Diagnostic ; let slugs = RefCell :: new (Vec :: new ()) ; let implementation = kind . each_variant (& mut structure , | mut builder , variant | { let preamble = builder . preamble (variant) ; let body = builder . body (variant) ; let init = match builder . slug . value_ref () { None => { span_err (builder . span , "diagnostic slug not specified") . help ("specify the slug as the first argument to the `#[diag(...)]` \
                            attribute, such as `#[diag(hir_analysis_example_error)]`" ,) . emit () ; return DiagnosticDeriveError :: ErrorHandled . to_compile_error () ; } Some (slug) if let Some (Mismatch { slug_name , crate_name , slug_prefix }) = Mismatch :: check (slug) => { span_err (slug . span () . unwrap () , "diagnostic slug and crate name do not match") . note (format ! ("slug is `{slug_name}` but the crate name is `{crate_name}`")) . help (format ! ("expected a slug starting with `{slug_prefix}_...`")) . emit () ; return DiagnosticDeriveError :: ErrorHandled . to_compile_error () ; } Some (slug) => { slugs . borrow_mut () . push (slug . clone ()) ; quote ! { let mut diag = rustc_errors :: Diag :: new (dcx , level , crate :: fluent_generated ::# slug) ; } } } ; let formatting_init = & builder . formatting_init ; quote ! { # init # formatting_init # preamble # body diag } }) ; # [allow (keyword_idents_2024)] let mut imp = structure . gen_impl (quote ! { gen impl <'_sess , G > rustc_errors :: Diagnostic <'_sess , G > for @ Self where G : rustc_errors :: EmissionGuarantee { # [track_caller] fn into_diag (self , dcx : rustc_errors :: DiagCtxtHandle <'_sess >, level : rustc_errors :: Level) -> rustc_errors :: Diag <'_sess , G > { # implementation } } }) ; for test in slugs . borrow () . iter () . map (| s | generate_test (s , & structure)) { imp . extend (test) ; } imp } }
    };
}

impl_5!();