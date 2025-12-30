// Generated macro for impl_41 (impl)
macro_rules! Depcrate_ansiimpl_41 {
() => {
// Module: crate::ansi
// Provides: {"impl_41"}
// Dependencies: {}
# [cfg (feature = "std")] impl Mul < f32 > for Rgb { type Output = Rgb ; fn mul (self , rhs : f32) -> Rgb { let result = Rgb { r : (f32 :: from (self . r) * rhs) . clamp (0.0 , 255.0) as u8 , g : (f32 :: from (self . g) * rhs) . clamp (0.0 , 255.0) as u8 , b : (f32 :: from (self . b) * rhs) . clamp (0.0 , 255.0) as u8 , } ; log :: trace ! ("Scaling RGB by {} from {:?} to {:?}" , rhs , self , result) ; result } }
};
}
