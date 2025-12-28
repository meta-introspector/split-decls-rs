macro_rules! macro_61 {
    () => {
        def_regs ! { Msp430 Msp430InlineAsmReg Msp430InlineAsmRegClass { r5 : reg = ["r5"] , r6 : reg = ["r6"] , r7 : reg = ["r7"] , r8 : reg = ["r8"] , r9 : reg = ["r9"] , r10 : reg = ["r10"] , r11 : reg = ["r11"] , r12 : reg = ["r12"] , r13 : reg = ["r13"] , r14 : reg = ["r14"] , r15 : reg = ["r15"] , # error = ["r0" , "pc"] => "the program counter cannot be used as an operand for inline asm" , # error = ["r1" , "sp"] => "the stack pointer cannot be used as an operand for inline asm" , # error = ["r2" , "sr"] => "the status register cannot be used as an operand for inline asm" , # error = ["r3" , "cg"] => "the constant generator cannot be used as an operand for inline asm" , # error = ["r4" , "fp"] => "the frame pointer cannot be used as an operand for inline asm" , } }
    };
}

macro_61!();