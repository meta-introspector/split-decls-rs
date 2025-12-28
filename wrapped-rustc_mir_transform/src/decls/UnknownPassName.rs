macro_rules! UnknownPassName {
    () => {
        # [derive (Diagnostic)] # [diag (mir_transform_unknown_pass_name)] pub (crate) struct UnknownPassName < 'a > { pub (crate) name : & 'a str , }
    };
}

UnknownPassName!();