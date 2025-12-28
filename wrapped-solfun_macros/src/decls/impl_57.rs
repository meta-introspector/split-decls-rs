macro_rules! deps {
    () => {
        ToolCallArgs!();
    };
}

macro_rules! impl_57 {
    () => {
        deps!();
        impl Parse for ToolCallArgs { fn parse (input : ParseStream) -> SynResult < Self > { let tool_name = input . parse () ? ; let _comma = input . parse () ? ; let args = input . parse () ? ; Ok (ToolCallArgs { tool_name , _comma , args }) } }
    };
}

impl_57!()