macro_rules! Magic {
    () => {
        # [doc = " \"Magic\" header values used in the zip spec to locate metadata records."] # [doc = ""] # [doc = " These values currently always take up a fixed four bytes, so we can parse and wrap them in this"] # [doc = " struct to enforce some small amount of type safety."] # [derive (Copy , Clone , Debug , PartialOrd , Ord , PartialEq , Eq , Hash)] # [repr (transparent)] pub (crate) struct Magic (u32) ;
    };
}

Magic!();