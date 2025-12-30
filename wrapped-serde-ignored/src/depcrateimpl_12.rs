// Generated macro for impl_12 (impl)
macro_rules! Depcrateimpl_12 {
() => {
// Module: crate
// Provides: {"impl_12"}
// Dependencies: {}
impl < 'a > Display for Path < 'a > { fn fmt (& self , formatter : & mut fmt :: Formatter) -> Result < () , fmt :: Error > { struct Parent < 'a > (& 'a Path < 'a >) ; impl < 'a > Display for Parent < 'a > { fn fmt (& self , formatter : & mut fmt :: Formatter) -> Result < () , fmt :: Error > { match * self . 0 { Path :: Root => Ok (()) , ref path => write ! (formatter , "{}." , path) , } } } match * self { Path :: Root => formatter . write_str (".") , Path :: Seq { parent , index } => write ! (formatter , "{}{}" , Parent (parent) , index) , Path :: Map { parent , ref key } => write ! (formatter , "{}{}" , Parent (parent) , key) , Path :: Some { parent } | Path :: NewtypeStruct { parent } | Path :: NewtypeVariant { parent } => write ! (formatter , "{}?" , Parent (parent)) , } } }
};
}
