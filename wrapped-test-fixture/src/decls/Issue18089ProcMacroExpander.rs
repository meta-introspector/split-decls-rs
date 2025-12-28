macro_rules! Issue18089ProcMacroExpander {
    () => {
        # [derive (Debug)] struct Issue18089ProcMacroExpander ;
    };
}

Issue18089ProcMacroExpander!()