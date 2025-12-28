macro_rules! BecauseRead {
    () => {
        # [allow (missing_copy_implementations , missing_debug_implementations)] pub enum BecauseRead { }
    };
}

BecauseRead!();