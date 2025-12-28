macro_rules! deps {
    () => {
        DefIdVisitorSkeleton!();
    };
}

macro_rules! DefIdVisitor {
    () => {
        deps!();
        # [doc = " Implemented to visit all `DefId`s in a type."] # [doc = " Visiting `DefId`s is useful because visibilities and reachabilities are attached to them."] # [doc = " The idea is to visit \"all components of a type\", as documented in"] # [doc = " <https://github.com/rust-lang/rfcs/blob/master/text/2145-type-privacy.md#how-to-determine-visibility-of-a-type>."] # [doc = " The default type visitor (`TypeVisitor`) does most of the job, but it has some shortcomings."] # [doc = " First, it doesn't have overridable `fn visit_trait_ref`, so we have to catch trait `DefId`s"] # [doc = " manually. Second, it doesn't visit some type components like signatures of fn types, or traits"] # [doc = " in `impl Trait`, see individual comments in `DefIdVisitorSkeleton::visit_ty`."] pub trait DefIdVisitor < 'tcx > { type Result : VisitorResult = () ; const SHALLOW : bool = false ; fn skip_assoc_tys (& self) -> bool { false } fn tcx (& self) -> TyCtxt < 'tcx > ; fn visit_def_id (& mut self , def_id : DefId , kind : & str , descr : & dyn fmt :: Display) -> Self :: Result ; # [doc = " Not overridden, but used to actually visit types and traits."] fn skeleton (& mut self) -> DefIdVisitorSkeleton < '_ , 'tcx , Self > { DefIdVisitorSkeleton { def_id_visitor : self , visited_opaque_tys : Default :: default () , dummy : Default :: default () , } } fn visit (& mut self , ty_fragment : impl TypeVisitable < TyCtxt < 'tcx > >) -> Self :: Result { ty_fragment . visit_with (& mut self . skeleton ()) } fn visit_trait (& mut self , trait_ref : TraitRef < 'tcx >) -> Self :: Result { self . skeleton () . visit_trait (trait_ref) } fn visit_predicates (& mut self , predicates : ty :: GenericPredicates < 'tcx >) -> Self :: Result { self . skeleton () . visit_clauses (predicates . predicates) } fn visit_clauses (& mut self , clauses : & [(ty :: Clause < 'tcx > , Span)]) -> Self :: Result { self . skeleton () . visit_clauses (clauses) } }
    };
}

DefIdVisitor!()