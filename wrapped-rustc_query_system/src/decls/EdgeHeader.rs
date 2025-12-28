macro_rules! EdgeHeader {
    () => {
        # [doc = " A packed representation of an edge's start index and byte width."] # [doc = ""] # [doc = " This is packed by stealing 2 bits from the start index, which means we only accommodate edge"] # [doc = " data arrays up to a quarter of our address space. Which seems fine."] # [derive (Debug , Clone , Copy)] struct EdgeHeader { repr : usize , num_edges : u32 , }
    };
}

EdgeHeader!();