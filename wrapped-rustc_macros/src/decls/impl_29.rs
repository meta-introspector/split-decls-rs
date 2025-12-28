macro_rules! deps {
    () => {
        SubdiagnosticDerive!();
        SubdiagnosticDeriveVariantBuilder!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl SubdiagnosticDerive { pub (crate) fn new () -> Self { let diag = format_ident ! ("diag") ; Self { diag } } pub (crate) fn into_tokens (self , mut structure : Structure < '_ >) -> TokenStream { let implementation = { let ast = structure . ast () ; let span = ast . span () . unwrap () ; match ast . data { syn :: Data :: Struct (..) | syn :: Data :: Enum (..) => () , syn :: Data :: Union (..) => { span_err (span , "`#[derive(Subdiagnostic)]` can only be used on structs and enums" ,) . emit () ; } } let is_enum = matches ! (ast . data , syn :: Data :: Enum (..)) ; if is_enum { for attr in & ast . attrs { if is_doc_comment (attr) { continue ; } span_err (attr . span () . unwrap () , "unsupported type attribute for subdiagnostic enum" ,) . emit () ; } } structure . bind_with (| _ | synstructure :: BindStyle :: Move) ; let variants_ = structure . each_variant (| variant | { let mut builder = SubdiagnosticDeriveVariantBuilder { parent : & self , variant , span , formatting_init : TokenStream :: new () , fields : build_field_mapping (variant) , span_field : None , applicability : None , has_suggestion_parts : false , has_subdiagnostic : false , is_enum , } ; builder . into_tokens () . unwrap_or_else (| v | v . to_compile_error ()) }) ; quote ! { match self { # variants_ } } } ; let diag = & self . diag ; # [allow (keyword_idents_2024)] let ret = structure . gen_impl (quote ! { gen impl rustc_errors :: Subdiagnostic for @ Self { fn add_to_diag < __G > (self , # diag : & mut rustc_errors :: Diag <'_ , __G >,) where __G : rustc_errors :: EmissionGuarantee , { # implementation } } }) ; ret } }
    };
}

impl_29!()