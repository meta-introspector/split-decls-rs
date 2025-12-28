macro_rules! deps {
    () => {
        MaybeItemFn!();
    };
}

macro_rules! instrument_speculative {
    () => {
        deps!();
        # [doc = " Instrument the function, without parsing the function body (instead using the raw tokens)."] fn instrument_speculative (args : attr :: InstrumentArgs , item : proc_macro :: TokenStream ,) -> proc_macro :: TokenStream { let input = syn :: parse_macro_input ! (item as MaybeItemFn) ; let instrumented_function_name = input . sig . ident . to_string () ; expand :: gen_function (input . as_ref () , args , instrumented_function_name . as_str () , None ,) . into () }
    };
}

instrument_speculative!()