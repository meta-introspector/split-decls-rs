macro_rules! DEP_NODE_WIDTH_BITS {
    () => {
        # [doc = " Number of bits we need to store the number of used bytes in a SerializedDepNodeIndex."] # [doc = " Note that wherever we encode byte widths like this we actually store the number of bytes used"] # [doc = " minus 1; for a 4-byte value we technically would have 5 widths to store, but using one byte to"] # [doc = " store zeroes (which are relatively rare) is a decent tradeoff to save a bit in our bitfields."] const DEP_NODE_WIDTH_BITS : usize = DEP_NODE_SIZE / 2 ;
    };
}

DEP_NODE_WIDTH_BITS!();