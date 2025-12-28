macro_rules! deps {
    () => {
        InlineAsmArch!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl AArch64InlineAsmReg { pub fn emit (self , out : & mut dyn fmt :: Write , _arch : InlineAsmArch , modifier : Option < char > ,) -> fmt :: Result { let (prefix , index) = if let Some (index) = self . reg_index () { (modifier . unwrap_or ('x') , index) } else if let Some (index) = self . vreg_index () { (modifier . unwrap_or ('v') , index) } else { return out . write_str (self . name ()) ; } ; assert ! (index < 32) ; write ! (out , "{prefix}{index}") } # [doc = " If the register is an integer register then return its index."] pub fn reg_index (self) -> Option < u32 > { use AArch64InlineAsmReg :: * ; Some (match self { x0 => 0 , x1 => 1 , x2 => 2 , x3 => 3 , x4 => 4 , x5 => 5 , x6 => 6 , x7 => 7 , x8 => 8 , x9 => 9 , x10 => 10 , x11 => 11 , x12 => 12 , x13 => 13 , x14 => 14 , x15 => 15 , x16 => 16 , x17 => 17 , x18 => 18 , x20 => 20 , x21 => 21 , x22 => 22 , x23 => 23 , x24 => 24 , x25 => 25 , x26 => 26 , x27 => 27 , x28 => 28 , x30 => 30 , _ => return None , }) } # [doc = " If the register is a vector register then return its index."] pub fn vreg_index (self) -> Option < u32 > { use AArch64InlineAsmReg :: * ; if self as u32 >= v0 as u32 && self as u32 <= v31 as u32 { return Some (self as u32 - v0 as u32) ; } None } }
    };
}

impl_11!();