macro_rules! Issue17479ProcMacroExpander {
    () => {
        # [derive (Debug)] struct Issue17479ProcMacroExpander ;
    };
}

Issue17479ProcMacroExpander!();