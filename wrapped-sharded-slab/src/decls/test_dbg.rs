macro_rules! test_dbg {
    () => {
        # [cfg (all (test , loom))] macro_rules ! test_dbg { ($ e : expr) => { match $ e { e => { test_println ! ("{} = {:?}" , stringify ! ($ e) , & e) ; e } } } ; }
    };
}

test_dbg!();