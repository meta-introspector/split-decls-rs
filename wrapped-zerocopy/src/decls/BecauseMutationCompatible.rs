macro_rules! BecauseMutationCompatible {
    () => {
        # [allow (missing_copy_implementations , missing_debug_implementations)] pub enum BecauseMutationCompatible { }
    };
}

BecauseMutationCompatible!();