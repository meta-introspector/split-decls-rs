macro_rules! test {
    () => {
        # [cfg (test)] # [cfg (feature = "std")] mod test ;
    };
}

test!();