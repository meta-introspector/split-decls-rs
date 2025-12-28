macro_rules! deps {
    () => {
        IngredientShard!();
        Configuration!();
    };
}

macro_rules! impl_180 {
    () => {
        deps!();
        impl < C : Configuration > Default for IngredientShard < C > { fn default () -> Self { Self { lru : LinkedList :: default () , key_map : hashbrown :: HashTable :: new () , } } }
    };
}

impl_180!();