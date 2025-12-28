macro_rules! ToolCallArgs {
    () => {
        struct ToolCallArgs { tool_name : LitStr , _comma : Token ! [,] , args : LitStr , }
    };
}

ToolCallArgs!();