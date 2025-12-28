macro_rules! ENOUGH {
    () => {
        # [doc = " Maximum size of the dynamic table.  The maximum number of code structures is"] # [doc = " 1924, which is the sum of 1332 for literal/length codes and 592 for distance"] # [doc = " codes.  These values were found by exhaustive searches using the program"] # [doc = " examples/enough.c found in the zlib distributions.  The arguments to that"] # [doc = " program are the number of symbols, the initial root table size, and the"] # [doc = " maximum bit length of a code.  \"enough 286 10 15\" for literal/length codes"] # [doc = " returns 1332, and \"enough 30 9 15\" for distance codes returns 592."] # [doc = " The initial root table size (10 or 9) is found in the fifth argument of the"] # [doc = " inflate_table() calls in inflate.c and infback.c.  If the root table size is"] # [doc = " changed, then these maximum sizes would be need to be recalculated and"] # [doc = " updated."] # [allow (unused)] pub (crate) const ENOUGH : usize = ENOUGH_LENS + ENOUGH_DISTS ;
    };
}

ENOUGH!()