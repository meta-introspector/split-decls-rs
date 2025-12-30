// Generated macro for Int (trait)
macro_rules! Depcrate_traitsInt {
() => {
// Module: crate::traits
// Provides: {"Int"}
// Dependencies: {}
# [doc = " Integer types."] # [allow (dead_code)] pub trait Int : Clone + Copy + fmt :: Debug + fmt :: Display + fmt :: LowerHex + ops :: Add < Output = Self > + ops :: Sub < Output = Self > + ops :: Shl < u32 , Output = Self > + ops :: Shr < u32 , Output = Self > + ops :: BitAnd < Output = Self > + ops :: BitOr < Output = Self > + ops :: Not < Output = Self > + ops :: AddAssign + ops :: BitAndAssign + ops :: BitOrAssign + From < u8 > + TryFrom < i8 > + TryFrom < u32 , Error : fmt :: Debug > + TryFrom < u64 , Error : fmt :: Debug > + TryFrom < u128 , Error : fmt :: Debug > + TryInto < u64 , Error : fmt :: Debug > + TryInto < u32 , Error : fmt :: Debug > + ToBigInt + PartialOrd + Integer + Send + 'static { type Signed : Int ; type Bytes : Default + AsMut < [u8] > ; const BITS : u32 ; const ZERO : Self ; const ONE : Self ; const MAX : Self ; fn to_signed (self) -> Self :: Signed ; fn wrapping_neg (self) -> Self ; fn trailing_zeros (self) -> u32 ; fn hex (self) -> String { format ! ("{self:x}") } }
};
}
