macro_rules! SuggestionKind {
    () => {
        # [doc = " Possible styles for suggestion subdiagnostics."] # [derive (Clone , Copy , PartialEq)] pub (super) enum SuggestionKind { Normal , Short , Hidden , Verbose , ToolOnly , }
    };
}

SuggestionKind!()