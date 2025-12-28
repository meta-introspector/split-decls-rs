macro_rules! deps {
    () => {
        InlineAsmArch!();
    };
}

macro_rules! impl_73 {
    () => {
        deps!();
        impl PowerPCInlineAsmReg { pub fn emit (self , out : & mut dyn fmt :: Write , _arch : InlineAsmArch , _modifier : Option < char > ,) -> fmt :: Result { macro_rules ! do_emit { ($ ($ (($ reg : ident , $ value : literal)) ,*;) *) => { out . write_str (match self { $ ($ (Self ::$ reg => $ value ,) *) * }) } ; } do_emit ! { (r0 , "0") , (r3 , "3") , (r4 , "4") , (r5 , "5") , (r6 , "6") , (r7 , "7") ; (r8 , "8") , (r9 , "9") , (r10 , "10") , (r11 , "11") , (r12 , "12") , (r13 , "13") , (r14 , "14") , (r15 , "15") ; (r16 , "16") , (r17 , "17") , (r18 , "18") , (r19 , "19") , (r20 , "20") , (r21 , "21") , (r22 , "22") , (r23 , "23") ; (r24 , "24") , (r25 , "25") , (r26 , "26") , (r27 , "27") , (r28 , "28") ; (f0 , "0") , (f1 , "1") , (f2 , "2") , (f3 , "3") , (f4 , "4") , (f5 , "5") , (f6 , "6") , (f7 , "7") ; (f8 , "8") , (f9 , "9") , (f10 , "10") , (f11 , "11") , (f12 , "12") , (f13 , "13") , (f14 , "14") , (f15 , "15") ; (f16 , "16") , (f17 , "17") , (f18 , "18") , (f19 , "19") , (f20 , "20") , (f21 , "21") , (f22 , "22") , (f23 , "23") ; (f24 , "24") , (f25 , "25") , (f26 , "26") , (f27 , "27") , (f28 , "28") , (f29 , "29") , (f30 , "30") , (f31 , "31") ; (v0 , "0") , (v1 , "1") , (v2 , "2") , (v3 , "3") , (v4 , "4") , (v5 , "5") , (v6 , "6") , (v7 , "7") ; (v8 , "8") , (v9 , "9") , (v10 , "10") , (v11 , "11") , (v12 , "12") , (v13 , "13") , (v14 , "14") , (v15 , "15") ; (v16 , "16") , (v17 , "17") , (v18 , "18") , (v19 , "19") , (v20 , "20") , (v21 , "21") , (v22 , "22") , (v23 , "23") ; (v24 , "24") , (v25 , "25") , (v26 , "26") , (v27 , "27") , (v28 , "28") , (v29 , "29") , (v30 , "30") , (v31 , "31") ; (cr , "cr") ; (cr0 , "0") , (cr1 , "1") , (cr2 , "2") , (cr3 , "3") , (cr4 , "4") , (cr5 , "5") , (cr6 , "6") , (cr7 , "7") ; (xer , "xer") ; } } pub fn overlapping_regs (self , mut cb : impl FnMut (PowerPCInlineAsmReg)) { macro_rules ! reg_conflicts { ($ ($ full : ident : $ ($ field : ident) *) ,*;) => { match self { $ (Self ::$ full => { cb (Self ::$ full) ; $ (cb (Self ::$ field) ;) * } $ (Self ::$ field) |* => { cb (Self ::$ full) ; cb (self) ; }) * r => cb (r) , } } ; } reg_conflicts ! { cr : cr0 cr1 cr2 cr3 cr4 cr5 cr6 cr7 ; } } }
    };
}

impl_73!()