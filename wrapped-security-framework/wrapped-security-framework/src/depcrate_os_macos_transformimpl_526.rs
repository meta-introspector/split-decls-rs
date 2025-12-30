// Generated macro for impl_526 (impl)
macro_rules! Depcrate_os_macos_transformimpl_526 {
() => {
// Module: crate::os::macos::transform
// Provides: {"impl_526"}
// Dependencies: {}
impl SecTransform { # [doc = " Sets an attribute of the transform."] pub fn set_attribute < T > (& mut self , key : & CFString , value : & T) -> Result < () , CFError > where T : TCFType { unsafe { let mut error = ptr :: null_mut () ; SecTransformSetAttribute (self . 0 , key . as_concrete_TypeRef () , value . as_CFTypeRef () , & mut error ,) ; if ! error . is_null () { return Err (CFError :: wrap_under_create_rule (error)) ; } Ok (()) } } # [doc = " Executes the transform."] # [doc = ""] # [doc = " The return type depends on the type of transform."] pub fn execute (& mut self) -> Result < CFType , CFError > { unsafe { let mut error = ptr :: null_mut () ; let result = SecTransformExecute (self . 0 , & mut error) ; if result . is_null () { return Err (CFError :: wrap_under_create_rule (error)) ; } Ok (CFType :: wrap_under_create_rule (result)) } } }
};
}
