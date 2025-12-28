macro_rules! deps {
    () => {
        InlineAsmArch!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl ArmInlineAsmReg { pub fn emit (self , out : & mut dyn fmt :: Write , _arch : InlineAsmArch , modifier : Option < char > ,) -> fmt :: Result { if let Some (modifier) = modifier { let index = self as u32 - Self :: q0 as u32 ; assert ! (index < 16) ; let index = index * 2 + (modifier == 'f') as u32 ; write ! (out , "d{index}") } else { out . write_str (self . name ()) } } pub fn overlapping_regs (self , mut cb : impl FnMut (ArmInlineAsmReg)) { cb (self) ; macro_rules ! reg_conflicts { ($ ($ q : ident : $ d0 : ident $ d1 : ident : $ s0 : ident $ s1 : ident $ s2 : ident $ s3 : ident) ,*; $ ($ q_high : ident : $ d0_high : ident $ d1_high : ident) ,*;) => { match self { $ (Self ::$ q => { cb (Self ::$ d0) ; cb (Self ::$ d1) ; cb (Self ::$ s0) ; cb (Self ::$ s1) ; cb (Self ::$ s2) ; cb (Self ::$ s3) ; } Self ::$ d0 => { cb (Self ::$ q) ; cb (Self ::$ s0) ; cb (Self ::$ s1) ; } Self ::$ d1 => { cb (Self ::$ q) ; cb (Self ::$ s2) ; cb (Self ::$ s3) ; } Self ::$ s0 | Self ::$ s1 => { cb (Self ::$ q) ; cb (Self ::$ d0) ; } Self ::$ s2 | Self ::$ s3 => { cb (Self ::$ q) ; cb (Self ::$ d1) ; }) * $ (Self ::$ q_high => { cb (Self ::$ d0_high) ; cb (Self ::$ d1_high) ; } Self ::$ d0_high | Self ::$ d1_high => { cb (Self ::$ q_high) ; }) * _ => { } , } } ; } reg_conflicts ! { q0 : d0 d1 : s0 s1 s2 s3 , q1 : d2 d3 : s4 s5 s6 s7 , q2 : d4 d5 : s8 s9 s10 s11 , q3 : d6 d7 : s12 s13 s14 s15 , q4 : d8 d9 : s16 s17 s18 s19 , q5 : d10 d11 : s20 s21 s22 s23 , q6 : d12 d13 : s24 s25 s26 s27 , q7 : d14 d15 : s28 s29 s30 s31 ; q8 : d16 d17 , q9 : d18 d19 , q10 : d20 d21 , q11 : d22 d23 , q12 : d24 d25 , q13 : d26 d27 , q14 : d28 d29 , q15 : d30 d31 ; } } }
    };
}

impl_21!();