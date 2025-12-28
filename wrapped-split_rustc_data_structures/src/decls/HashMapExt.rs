macro_rules! HashMapExt {
    () => {
        pub trait HashMapExt < K , V > { # [doc = " Same as HashMap::insert, but it may panic if there's already an"] # [doc = " entry for `key` with a value not equal to `value`"] fn insert_same (& mut self , key : K , value : V) ; }
    };
}

HashMapExt!();