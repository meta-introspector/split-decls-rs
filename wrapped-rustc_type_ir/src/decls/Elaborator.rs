macro_rules! deps {
    () => {
        Interner!();
        ElaborateSized!();
        Binder!();
        Filter!();
        PredicateKind!();
    };
}

macro_rules! Elaborator {
    () => {
        deps!();
        # [doc = " \"Elaboration\" is the process of identifying all the predicates that"] # [doc = " are implied by a source predicate. Currently, this basically means"] # [doc = " walking the \"supertraits\" and other similar assumptions. For example,"] # [doc = " if we know that `T: Ord`, the elaborator would deduce that `T: PartialOrd`"] # [doc = " holds as well. Similarly, if we have `trait Foo: 'static`, and we know that"] # [doc = " `T: Foo`, then we know that `T: 'static`."] pub struct Elaborator < I : Interner , O > { cx : I , stack : Vec < O > , visited : HashSet < ty :: Binder < I , ty :: PredicateKind < I > > > , mode : Filter , elaborate_sized : ElaborateSized , }
    };
}

Elaborator!()