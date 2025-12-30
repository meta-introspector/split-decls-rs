// Generated macro for thin_vec (macro)
macro_rules! Depcratethin_vec {
() => {
// Module: crate
// Provides: {"thin_vec"}
// Dependencies: {}
# [doc = " Creates a `ThinVec` containing the arguments."] # [doc = ""] # [cfg_attr (not (feature = "gecko-ffi") , doc = "```")] # [cfg_attr (feature = "gecko-ffi" , doc = "```ignore")] # [doc = " #[macro_use] extern crate thin_vec;"] # [doc = ""] # [doc = " fn main() {"] # [doc = "     let v = thin_vec![1, 2, 3];"] # [doc = "     assert_eq!(v.len(), 3);"] # [doc = "     assert_eq!(v[0], 1);"] # [doc = "     assert_eq!(v[1], 2);"] # [doc = "     assert_eq!(v[2], 3);"] # [doc = ""] # [doc = "     let v = thin_vec![1; 3];"] # [doc = "     assert_eq!(v, [1, 1, 1]);"] # [doc = " }"] # [doc = " ```"] # [macro_export] macro_rules ! thin_vec { (@ UNIT $ ($ t : tt) *) => (()) ; ($ elem : expr ; $ n : expr) => ({ let mut vec = $ crate :: ThinVec :: new () ; vec . resize ($ n , $ elem) ; vec }) ; () => { $ crate :: ThinVec :: new () } ; ($ ($ x : expr) ,*) => ({ let len = [$ ($ crate :: thin_vec ! (@ UNIT $ x)) ,*] . len () ; let mut vec = $ crate :: ThinVec :: with_capacity (len) ; $ (vec . push ($ x) ;) * vec }) ; ($ ($ x : expr ,) *) => ($ crate :: thin_vec ! [$ ($ x) ,*]) ; }
};
}
