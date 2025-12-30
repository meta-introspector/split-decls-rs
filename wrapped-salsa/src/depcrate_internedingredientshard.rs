// Generated macro for IngredientShard (struct)
macro_rules! Depcrate_internedIngredientShard {
() => {
// Module: crate::interned
// Provides: {"IngredientShard"}
// Dependencies: {}
struct IngredientShard < C : Configuration > { # [doc = " Maps from data to the existing interned ID for that data."] # [doc = ""] # [doc = " This doesn't hold the fields themselves to save memory, instead it points"] # [doc = " to the slot ID."] key_map : hashbrown :: HashTable < Id > , # [doc = " An intrusive linked list for LRU."] lru : LinkedList < ValueAdapter < C > > , }
};
}
