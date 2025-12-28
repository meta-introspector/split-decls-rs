macro_rules! deps {
    () => {
        SubdiagnosticKind!();
    };
}

macro_rules! impl_67 {
    () => {
        deps!();
        impl quote :: IdentFragment for SubdiagnosticKind { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { SubdiagnosticKind :: Label => write ! (f , "label") , SubdiagnosticKind :: Note => write ! (f , "note") , SubdiagnosticKind :: NoteOnce => write ! (f , "note_once") , SubdiagnosticKind :: Help => write ! (f , "help") , SubdiagnosticKind :: HelpOnce => write ! (f , "help_once") , SubdiagnosticKind :: Warn => write ! (f , "warn") , SubdiagnosticKind :: Suggestion { .. } => write ! (f , "suggestions_with_style") , SubdiagnosticKind :: MultipartSuggestion { .. } => { write ! (f , "multipart_suggestion_with_style") } } } fn span (& self) -> Option < proc_macro2 :: Span > { None } }
    };
}

impl_67!()