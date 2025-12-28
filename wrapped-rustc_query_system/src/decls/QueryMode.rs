macro_rules! QueryMode {
    () => {
        # [derive (Debug)] pub enum QueryMode { Get , Ensure { check_cache : bool } , }
    };
}

QueryMode!();