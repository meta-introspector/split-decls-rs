// Generated macro for impl_145 (impl)
macro_rules! Depcrateimpl_145 {
() => {
// Module: crate
// Provides: {"impl_145"}
// Dependencies: {}
impl Display for ConstValue { fn fmt (& self , f : & mut Formatter < '_ >) -> fmt :: Result { match self { Self :: Number (num) => write ! (f , "{}" , * num) , Self :: String (val) => write_quoted (val , f) , Self :: Boolean (true) => f . write_str ("true") , Self :: Boolean (false) => f . write_str ("false") , Self :: Binary (bytes) => write_binary (bytes , f) , Self :: Null => f . write_str ("null") , Self :: Enum (name) => f . write_str (name) , Self :: List (items) => write_list (items , f) , Self :: Object (map) => write_object (map , f) , } } }
};
}
