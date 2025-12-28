macro_rules! deps {
    () => {
        CrateItems!();
    };
}

macro_rules! all_local_items {
    () => {
        deps!();
        # [doc = " Retrieve all items in the local crate that have a MIR associated with them."] pub fn all_local_items () -> CrateItems { with (| cx | cx . all_local_items ()) }
    };
}

all_local_items!();