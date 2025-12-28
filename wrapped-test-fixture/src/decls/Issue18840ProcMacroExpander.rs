macro_rules! Issue18840ProcMacroExpander {
    () => {
        # [derive (Debug)] struct Issue18840ProcMacroExpander ;
    };
}

Issue18840ProcMacroExpander!()