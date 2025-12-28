macro_rules! deps {
    () => {
        ItemFollowingInnerAttr!();
    };
}

macro_rules! InvalidAttrAtCrateLevel {
    () => {
        deps!();
        pub (crate) struct InvalidAttrAtCrateLevel { pub span : Span , pub sugg_span : Option < Span > , pub name : Symbol , pub item : Option < ItemFollowingInnerAttr > , }
    };
}

InvalidAttrAtCrateLevel!()