macro_rules! macro_51 {
    () => {
        def_regs ! { M68k M68kInlineAsmReg M68kInlineAsmRegClass { d0 : reg , reg_data = ["d0"] , d1 : reg , reg_data = ["d1"] , d2 : reg , reg_data = ["d2"] , d3 : reg , reg_data = ["d3"] , d4 : reg , reg_data = ["d4"] , d5 : reg , reg_data = ["d5"] , d6 : reg , reg_data = ["d6"] , d7 : reg , reg_data = ["d7"] , a0 : reg , reg_addr = ["a0"] , a1 : reg , reg_addr = ["a1"] , a2 : reg , reg_addr = ["a2"] , a3 : reg , reg_addr = ["a3"] , # error = ["a4"] => "a4 is used internally by LLVM and cannot be used as an operand for inline asm" , # error = ["a5" , "bp"] => "a5 is used internally by LLVM and cannot be used as an operand for inline asm" , # error = ["a6" , "fp"] => "a6 is used internally by LLVM and cannot be used as an operand for inline asm" , # error = ["a7" , "sp" , "usp" , "ssp" , "isp"] => "the stack pointer cannot be used as an operand for inline asm" , } }
    };
}

macro_51!()