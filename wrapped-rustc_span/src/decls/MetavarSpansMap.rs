macro_rules! deps {
    () => {
        Span!();
    };
}

macro_rules! MetavarSpansMap {
    () => {
        deps!();
        # [derive (Default)] pub struct MetavarSpansMap (FreezeLock < UnordMap < Span , (Span , bool) > >) ;
    };
}

MetavarSpansMap!()