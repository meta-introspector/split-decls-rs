macro_rules! test {
    () => {
        # [cfg (test)] mod test { use super :: * ; # [test] fn existing () { assert_eq ! ("SIGTERM" , signal_name (SIGTERM) . unwrap ()) ; } # [test] fn unknown () { assert ! (signal_name (128) . is_none ()) ; } }
    };
}

test!();