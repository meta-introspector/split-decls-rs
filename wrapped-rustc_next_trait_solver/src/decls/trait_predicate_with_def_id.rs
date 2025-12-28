macro_rules! trait_predicate_with_def_id {
    () => {
        # [doc = " Small helper function to change the `def_id` of a trait predicate - this is not normally"] # [doc = " something that you want to do, as different traits will require different args and so making"] # [doc = " it easy to change the trait is something of a footgun, but it is useful in the narrow"] # [doc = " circumstance of changing from `MetaSized` to `Sized`, which happens as part of the lazy"] # [doc = " elaboration of sizedness candidates."] # [inline (always)] fn trait_predicate_with_def_id < I : Interner > (cx : I , clause : ty :: Binder < I , ty :: TraitPredicate < I > > , did : I :: TraitId ,) -> I :: Clause { clause . map_bound (| c | TraitPredicate { trait_ref : TraitRef :: new_from_args (cx , did , c . trait_ref . args) , polarity : c . polarity , }) . upcast (cx) }
    };
}

trait_predicate_with_def_id!()