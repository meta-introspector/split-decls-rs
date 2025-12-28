macro_rules! UnrecognizedReprError {
    () => {
        # [doc = " The representation hint could not be parsed or was unrecognized."] struct UnrecognizedReprError ;
    };
}

UnrecognizedReprError!();