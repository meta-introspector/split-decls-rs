macro_rules! deps {
    () => {
        TextSize!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        # [doc = " Methods to act like a primitive integer type, where reasonably applicable."] impl TextSize { # [doc = " Checked addition. Returns `None` if overflow occurred."] # [inline] pub const fn checked_add (self , rhs : TextSize) -> Option < TextSize > { match self . raw . checked_add (rhs . raw) { Some (raw) => Some (TextSize { raw }) , None => None , } } # [doc = " Checked subtraction. Returns `None` if overflow occurred."] # [inline] pub const fn checked_sub (self , rhs : TextSize) -> Option < TextSize > { match self . raw . checked_sub (rhs . raw) { Some (raw) => Some (TextSize { raw }) , None => None , } } }
    };
}

impl_22!();