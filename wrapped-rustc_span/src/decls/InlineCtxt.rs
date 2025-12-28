macro_rules! InlineCtxt {
    () => {
        # [derive (Clone , Copy)] struct InlineCtxt { lo : u32 , len : u16 , ctxt : u16 , }
    };
}

InlineCtxt!();