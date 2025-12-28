macro_rules! MetavarSpansMap {
    () => {
        # [derive (Default)] pub struct MetavarSpansMap (FreezeLock < UnordMap < Span , (Span , bool) > >) ;
    };
}

MetavarSpansMap!()