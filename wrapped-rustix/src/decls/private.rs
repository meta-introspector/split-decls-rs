macro_rules! deps {
    () => {
        Integer!();
    };
}

macro_rules! private {
    () => {
        deps!();
        mod private { pub trait Sealed : Copy { type Unsigned : super :: Integer ; fn as_unsigned (self) -> (bool , Self :: Unsigned) ; fn eq_zero (self) -> bool ; fn div_mod_10 (& mut self) -> u8 ; } macro_rules ! impl_unsigned { ($ ($ ty : ty) +) => { $ (impl Sealed for $ ty { type Unsigned = $ ty ; # [inline] fn as_unsigned (self) -> (bool , $ ty) { (false , self) } # [inline] fn eq_zero (self) -> bool { self == 0 } # [inline] fn div_mod_10 (& mut self) -> u8 { let result = (* self % 10) as u8 ; * self /= 10 ; result } }) + } } macro_rules ! impl_signed { ($ ($ signed : ty : $ unsigned : ty) +) => { $ (impl Sealed for $ signed { type Unsigned = $ unsigned ; # [inline] fn as_unsigned (self) -> (bool , $ unsigned) { if self >= 0 { (false , self as $ unsigned) } else { (true , ! (self as $ unsigned) + 1) } } # [inline] fn eq_zero (self) -> bool { unimplemented ! () } # [inline] fn div_mod_10 (& mut self) -> u8 { unimplemented ! () } }) + } } impl_unsigned ! (u8 u16 u32 u64) ; impl_signed ! (i8 : u8 i16 : u16 i32 : u32 i64 : u64) ; # [cfg (any (target_pointer_width = "16" , target_pointer_width = "32" , target_pointer_width = "64"))] const _ : () = { impl_unsigned ! (usize) ; impl_signed ! (isize : usize) ; } ; }
    };
}

private!()