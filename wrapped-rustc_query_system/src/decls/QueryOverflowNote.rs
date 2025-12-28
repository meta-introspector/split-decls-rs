macro_rules! QueryOverflowNote {
    () => {
        # [derive (Subdiagnostic)] # [note (query_system_overflow_note)] pub struct QueryOverflowNote { pub desc : String , pub depth : usize , }
    };
}

QueryOverflowNote!();