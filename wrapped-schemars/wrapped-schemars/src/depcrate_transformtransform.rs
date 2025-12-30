// Generated macro for Transform (trait)
macro_rules! Depcrate_transformTransform {
() => {
// Module: crate::transform
// Provides: {"Transform"}
// Dependencies: {}
# [doc = " Trait used to modify a constructed schema and optionally its subschemas."] # [doc = ""] # [doc = " See the [module documentation](self) for more details on implementing this trait."] pub trait Transform { # [doc = " Applies the transform to the given [`Schema`]."] # [doc = ""] # [doc = " When overriding this method, you may want to call the [`transform_subschemas`] function to"] # [doc = " also transform any subschemas."] fn transform (& mut self , schema : & mut Schema) ; # [doc (hidden)] fn _debug_type_name (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { f . write_str (core :: any :: type_name :: < Self > ()) } }
};
}
