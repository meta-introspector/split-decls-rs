macro_rules! deps {
    () => {
        GenericArg!();
        TypeWalkerStack!();
        Interner!();
    };
}

macro_rules! TypeWalker {
    () => {
        deps!();
        # [doc = " An iterator for walking the type tree."] # [doc = ""] # [doc = " It's very easy to produce a deeply"] # [doc = " nested type tree with a lot of"] # [doc = " identical subtrees. In order to work efficiently"] # [doc = " in this situation walker only visits each type once."] # [doc = " It maintains a set of visited types and"] # [doc = " skips any types that are already there."] pub struct TypeWalker < I : Interner > { stack : TypeWalkerStack < I > , last_subtree : usize , pub visited : SsoHashSet < I :: GenericArg > , }
    };
}

TypeWalker!()