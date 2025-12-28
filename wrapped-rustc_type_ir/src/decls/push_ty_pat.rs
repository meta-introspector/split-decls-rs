macro_rules! deps {
    () => {
        Interner!();
        PatternKind!();
        TypeWalkerStack!();
    };
}

macro_rules! push_ty_pat {
    () => {
        deps!();
        fn push_ty_pat < I : Interner > (stack : & mut TypeWalkerStack < I > , pat : I :: Pat) { match pat . kind () { ty :: PatternKind :: Range { start , end } => { stack . push (end . into ()) ; stack . push (start . into ()) ; } ty :: PatternKind :: Or (pats) => { for pat in pats . iter () { push_ty_pat :: < I > (stack , pat) } } } }
    };
}

push_ty_pat!();