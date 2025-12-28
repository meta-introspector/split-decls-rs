macro_rules! ComesFromAllowExpect {
    () => {
        # [doc = " Determine if a work from the worklist is coming from a `#[allow]`"] # [doc = " or a `#[expect]` of `dead_code`"] # [derive (Debug , Copy , Clone , Eq , PartialEq , Hash)] enum ComesFromAllowExpect { Yes , No , }
    };
}

ComesFromAllowExpect!();