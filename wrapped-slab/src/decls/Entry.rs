macro_rules! Entry {
    () => {
        # [derive (Clone)] enum Entry < T > { Vacant (usize) , Occupied (T) , }
    };
}

Entry!()