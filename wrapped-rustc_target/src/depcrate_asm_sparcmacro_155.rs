// Generated macro for macro_155 (macro)
macro_rules! Depcrate_asm_sparcmacro_155 {
() => {
// Module: crate::asm::sparc
// Provides: {"macro_155"}
// Dependencies: {}
def_regs ! { Sparc SparcInlineAsmReg SparcInlineAsmRegClass { r2 : reg = ["r2" , "g2"] , r3 : reg = ["r3" , "g3"] , r4 : reg = ["r4" , "g4"] , r5 : reg = ["r5" , "g5"] % reserved_g5 , r8 : reg = ["r8" , "o0"] , r9 : reg = ["r9" , "o1"] , r10 : reg = ["r10" , "o2"] , r11 : reg = ["r11" , "o3"] , r12 : reg = ["r12" , "o4"] , r13 : reg = ["r13" , "o5"] , r15 : reg = ["r15" , "o7"] , r16 : reg = ["r16" , "l0"] , r17 : reg = ["r17" , "l1"] , r18 : reg = ["r18" , "l2"] , r19 : reg = ["r19" , "l3"] , r20 : reg = ["r20" , "l4"] , r21 : reg = ["r21" , "l5"] , r22 : reg = ["r22" , "l6"] , r23 : reg = ["r23" , "l7"] , r24 : reg = ["r24" , "i0"] , r25 : reg = ["r25" , "i1"] , r26 : reg = ["r26" , "i2"] , r27 : reg = ["r27" , "i3"] , r28 : reg = ["r28" , "i4"] , r29 : reg = ["r29" , "i5"] , y : yreg = ["y"] , # error = ["r0" , "g0"] => "g0 is always zero and cannot be used as an operand for inline asm" , # error = ["r1" , "g1"] => "reserved by LLVM and cannot be used as an operand for inline asm" , # error = ["r6" , "g6" , "r7" , "g7"] => "reserved for system and cannot be used as an operand for inline asm" , # error = ["sp" , "r14" , "o6"] => "the stack pointer cannot be used as an operand for inline asm" , # error = ["fp" , "r30" , "i6"] => "the frame pointer cannot be used as an operand for inline asm" , # error = ["r31" , "i7"] => "the return address register cannot be used as an operand for inline asm" , } }
};
}
