macro_rules! Active {
    () => {
        # [doc = " Stores active entries (added and not yet removed)."] # [derive (Default)] struct Active { map : IndexMap < usize , u32 > , prev_value : u32 , }
    };
}

Active!();