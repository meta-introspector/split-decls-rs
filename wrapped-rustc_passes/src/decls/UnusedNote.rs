macro_rules! UnusedNote {
    () => {
        # [derive (Subdiagnostic)] pub (crate) enum UnusedNote { # [note (passes_unused_empty_lints_note)] EmptyList { name : Symbol } , # [note (passes_unused_no_lints_note)] NoLints { name : Symbol } , # [note (passes_unused_default_method_body_const_note)] DefaultMethodBodyConst , # [note (passes_unused_linker_messages_note)] LinkerMessagesBinaryCrateOnly , }
    };
}

UnusedNote!()