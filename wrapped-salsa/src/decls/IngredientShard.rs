macro_rules! deps {
    () => {
        Id!();
        Configuration!();
    };
}

macro_rules! IngredientShard {
    () => {
        deps!();
        struct IngredientShard < C : Configuration > { # [doc = " Maps from data to the existing interned ID for that data."] # [doc = ""] # [doc = " This doesn't hold the fields themselves to save memory, instead it points"] # [doc = " to the slot ID."] key_map : hashbrown :: HashTable < Id > , # [doc = " An intrusive linked list for LRU."] lru : LinkedList < ValueAdapter < C > > , }
    };
}

IngredientShard!();