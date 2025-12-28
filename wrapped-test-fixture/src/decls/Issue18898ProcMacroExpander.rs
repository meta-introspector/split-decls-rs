macro_rules! Issue18898ProcMacroExpander {
    () => {
        # [derive (Debug)] struct Issue18898ProcMacroExpander ;
    };
}

Issue18898ProcMacroExpander!();