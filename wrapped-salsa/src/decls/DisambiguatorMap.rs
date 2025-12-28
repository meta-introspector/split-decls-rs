macro_rules! deps {
    () => {
        IdentityHash!();
        Disambiguator!();
    };
}

macro_rules! DisambiguatorMap {
    () => {
        deps!();
        # [derive (Default , Debug)] pub (crate) struct DisambiguatorMap { map : hashbrown :: HashMap < IdentityHash , Disambiguator , () > , }
    };
}

DisambiguatorMap!();