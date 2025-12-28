macro_rules! CallToDeprecatedSafeFnRequiresUnsafeSub {
    () => {
        # [derive (Subdiagnostic)] # [multipart_suggestion (mir_build_suggestion , applicability = "machine-applicable")] pub (crate) struct CallToDeprecatedSafeFnRequiresUnsafeSub { pub (crate) start_of_line_suggestion : String , # [suggestion_part (code = "{start_of_line_suggestion}")] pub (crate) start_of_line : Span , # [suggestion_part (code = "unsafe {{ ")] pub (crate) left : Span , # [suggestion_part (code = " }}")] pub (crate) right : Span , }
    };
}

CallToDeprecatedSafeFnRequiresUnsafeSub!()