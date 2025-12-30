// Generated macro for item (macro)
macro_rules! Depcrate_serdeitem {
() => {
// Module: crate::serde
// Provides: {"item"}
// Dependencies: {}
# [doc = " Consume the next item in a sequence."] macro_rules ! item { ($ seq : expr , $ name : literal) => { $ seq . next_element () ? . ok_or_else (|| < A :: Error as serde_core :: de :: Error >:: custom (concat ! ("expected " , $ name))) } ; }
};
}
