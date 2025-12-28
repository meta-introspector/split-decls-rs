macro_rules! Errors {
    () => {
        # [derive (Default)] struct Errors { list : Vec < syn :: Error > , }
    };
}

Errors!();