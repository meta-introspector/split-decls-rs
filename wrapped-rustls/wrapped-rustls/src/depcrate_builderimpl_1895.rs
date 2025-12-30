// Generated macro for impl_1895 (impl)
macro_rules! Depcrate_builderimpl_1895 {
() => {
// Module: crate::builder
// Provides: {"impl_1895"}
// Dependencies: {}
impl < Side : ConfigSide , State : fmt :: Debug > fmt :: Debug for ConfigBuilder < Side , State > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let side_name = core :: any :: type_name :: < Side > () ; let (ty , _) = side_name . split_once ('<') . unwrap_or ((side_name , "")) ; let (_ , name) = ty . rsplit_once ("::") . unwrap_or (("" , ty)) ; f . debug_struct (& format ! ("ConfigBuilder<{name}, _>" ,)) . field ("state" , & self . state) . finish () } }
};
}
