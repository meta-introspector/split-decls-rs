macro_rules! deps {
    () => {
        Result!();
        Patchwork!();
        SourceFileRuntime!();
        Inline!();
        Span!();
    };
}

macro_rules! impl_68 {
    () => {
        deps!();
        impl SourceFileRuntime { fn new (inline : & Inline) -> std :: io :: Result < SourceFileRuntime > { let path = inline . position . file . clone () ; let original_text = std :: fs :: read_to_string (& path) ? ; let patchwork = Patchwork :: new (original_text . clone ()) ; Ok (SourceFileRuntime { path , original_text , patchwork , }) } fn update (& mut self , actual : & str , inline : & Inline) -> std :: io :: Result < () > { let span = Span :: from_pos (& inline . position , & self . original_text) ; let patch = format_patch (actual) ; self . patchwork . patch (span . literal_range , & patch) ? ; std :: fs :: write (& inline . position . file , & self . patchwork . text) } }
    };
}

impl_68!();