macro_rules! deps {
    () => {
        Mismatch!();
        DiagnosticDeriveError!();
        LintDiagnosticDerive!();
        DiagnosticDeriveKind!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl < 'a > LintDiagnosticDerive < 'a > { pub (crate) fn new (structure : Structure < 'a >) -> Self { Self { structure } } pub (crate) fn into_tokens (self) -> TokenStream { let LintDiagnosticDerive { mut structure } = self ; let kind = DiagnosticDeriveKind :: LintDiagnostic ; let slugs = RefCell :: new (Vec :: new ()) ; let implementation = kind . each_variant (& mut structure , | mut builder , variant | { let preamble = builder . preamble (variant) ; let body = builder . body (variant) ; let primary_message = match builder . slug . value_ref () { None => { span_err (builder . span , "diagnostic slug not specified") . help ("specify the slug as the first argument to the attribute, such as \
                            `#[diag(compiletest_example)]`" ,) . emit () ; DiagnosticDeriveError :: ErrorHandled . to_compile_error () } Some (slug) if let Some (Mismatch { slug_name , crate_name , slug_prefix }) = Mismatch :: check (slug) => { span_err (slug . span () . unwrap () , "diagnostic slug and crate name do not match") . note (format ! ("slug is `{slug_name}` but the crate name is `{crate_name}`")) . help (format ! ("expected a slug starting with `{slug_prefix}_...`")) . emit () ; DiagnosticDeriveError :: ErrorHandled . to_compile_error () } Some (slug) => { slugs . borrow_mut () . push (slug . clone ()) ; quote ! { diag . primary_message (crate :: fluent_generated ::# slug) ; } } } ; let formatting_init = & builder . formatting_init ; quote ! { # primary_message # preamble # formatting_init # body diag } }) ; # [allow (keyword_idents_2024)] let mut imp = structure . gen_impl (quote ! { gen impl <'__a > rustc_errors :: LintDiagnostic <'__a , () > for @ Self { # [track_caller] fn decorate_lint <'__b > (self , diag : &'__b mut rustc_errors :: Diag <'__a , () >) { # implementation ; } } }) ; for test in slugs . borrow () . iter () . map (| s | generate_test (s , & structure)) { imp . extend (test) ; } imp } }
    };
}

impl_7!()