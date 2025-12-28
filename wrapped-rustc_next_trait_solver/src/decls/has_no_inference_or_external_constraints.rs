macro_rules! has_no_inference_or_external_constraints {
    () => {
        fn has_no_inference_or_external_constraints < I : Interner > (response : ty :: Canonical < I , Response < I > > ,) -> bool { let ExternalConstraintsData { ref region_constraints , ref opaque_types , ref normalization_nested_goals , } = * response . value . external_constraints ; response . value . var_values . is_identity () && region_constraints . is_empty () && opaque_types . is_empty () && normalization_nested_goals . is_empty () }
    };
}

has_no_inference_or_external_constraints!()