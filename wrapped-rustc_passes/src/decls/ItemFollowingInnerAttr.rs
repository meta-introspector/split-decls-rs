macro_rules! ItemFollowingInnerAttr {
    () => {
        # [derive (Clone , Copy)] pub (crate) struct ItemFollowingInnerAttr { pub span : Span , pub kind : & 'static str , }
    };
}

ItemFollowingInnerAttr!()