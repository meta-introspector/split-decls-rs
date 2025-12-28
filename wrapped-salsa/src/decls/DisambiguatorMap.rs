macro_rules! deps {
    () => {
        Disambiguator!();
        IdentityHash!();
    };
}

macro_rules! DisambiguatorMap {
    () => {
        deps!();
        # [derive (Default , Debug)] pub (crate) struct DisambiguatorMap { map : hashbrown :: HashMap < IdentityHash , Disambiguator , () > , }
    };
}

DisambiguatorMap!()