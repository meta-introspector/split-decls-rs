macro_rules! deps {
    () => {
        HashMapExt!();
    };
}

macro_rules! impl_573 {
    () => {
        deps!();
        impl < K : Eq + Hash , V : Eq , S : BuildHasher > HashMapExt < K , V > for HashMap < K , V , S > { fn insert_same (& mut self , key : K , value : V) { self . entry (key) . and_modify (| old | assert ! (* old == value)) . or_insert (value) ; } }
    };
}

impl_573!();