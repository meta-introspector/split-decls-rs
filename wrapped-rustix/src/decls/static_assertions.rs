macro_rules! static_assertions {
    () => {
        # [cfg (all (test , not (static_assertions)))] # [macro_use] # [allow (unused_imports)] mod static_assertions ;
    };
}

static_assertions!();