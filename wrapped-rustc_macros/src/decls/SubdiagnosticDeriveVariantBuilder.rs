macro_rules! deps {
    () => {
        SubdiagnosticDerive!();
        SpannedOption!();
        FieldMap!();
    };
}

macro_rules! SubdiagnosticDeriveVariantBuilder {
    () => {
        deps!();
        # [doc = " Tracks persistent information required for building up the call to add to the diagnostic"] # [doc = " for the final generated method. This is a separate struct to `SubdiagnosticDerive`"] # [doc = " only to be able to destructure and split `self.builder` and the `self.structure` up to avoid a"] # [doc = " double mut borrow later on."] struct SubdiagnosticDeriveVariantBuilder < 'parent , 'a > { # [doc = " The identifier to use for the generated `Diag` instance."] parent : & 'parent SubdiagnosticDerive , # [doc = " Info for the current variant (or the type if not an enum)."] variant : & 'a VariantInfo < 'a > , # [doc = " Span for the entire type."] span : proc_macro :: Span , # [doc = " Initialization of format strings for code suggestions."] formatting_init : TokenStream , # [doc = " Store a map of field name to its corresponding field. This is built on construction of the"] # [doc = " derive builder."] fields : FieldMap , # [doc = " Identifier for the binding to the `#[primary_span]` field."] span_field : SpannedOption < proc_macro2 :: Ident > , # [doc = " The binding to the `#[applicability]` field, if present."] applicability : SpannedOption < TokenStream > , # [doc = " Set to true when a `#[suggestion_part]` field is encountered, used to generate an error"] # [doc = " during finalization if still `false`."] has_suggestion_parts : bool , # [doc = " Set to true when a `#[subdiagnostic]` field is encountered, used to suppress the error"] # [doc = " emitted when no subdiagnostic kinds are specified on the variant itself."] has_subdiagnostic : bool , # [doc = " Set to true when this variant is an enum variant rather than just the body of a struct."] is_enum : bool , }
    };
}

SubdiagnosticDeriveVariantBuilder!();