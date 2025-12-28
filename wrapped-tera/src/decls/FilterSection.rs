macro_rules! deps {
    () => {
        FunctionCall!();
        Node!();
    };
}

macro_rules! FilterSection {
    () => {
        deps!();
        # [doc = " A filter section node `{{ filter name(param=\"value\") }} content {{ endfilter }}`"] # [derive (Clone , Debug , PartialEq)] pub struct FilterSection { # [doc = " The filter call itsel"] pub filter : FunctionCall , # [doc = " The filter body"] pub body : Vec < Node > , }
    };
}

FilterSection!()