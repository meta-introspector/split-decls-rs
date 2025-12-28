macro_rules! deps {
    () => {
        MatchTreeBranch!();
    };
}

macro_rules! BuiltMatchTree {
    () => {
        deps!();
        # [doc = " The result of generating MIR for a pattern-matching expression. Each input branch/arm/pattern"] # [doc = " gives rise to an output `MatchTreeBranch`. If one of the patterns matches, we branch to the"] # [doc = " corresponding `success_block`. If none of the patterns matches, we branch to `otherwise_block`."] # [doc = ""] # [doc = " Each branch is made of one of more sub-branches, corresponding to or-patterns. E.g."] # [doc = " ```ignore(illustrative)"] # [doc = " match foo {"] # [doc = "     (x, false) | (false, x) => {}"] # [doc = "     (true, true) => {}"] # [doc = " }"] # [doc = " ```"] # [doc = " Here the first arm gives the first `MatchTreeBranch`, which has two sub-branches, one for each"] # [doc = " alternative of the or-pattern. They are kept separate because each needs to bind `x` to a"] # [doc = " different place."] # [derive (Debug , Clone)] pub (crate) struct BuiltMatchTree < 'tcx > { branches : Vec < MatchTreeBranch < 'tcx > > , otherwise_block : BasicBlock , # [doc = " If any of the branches had a guard, we collect here the places and locals to fakely borrow"] # [doc = " to ensure match guards can't modify the values as we match them. For more details, see"] # [doc = " [`util::collect_fake_borrows`]."] fake_borrow_temps : Vec < (Place < 'tcx > , Local , FakeBorrowKind) > , }
    };
}

BuiltMatchTree!();