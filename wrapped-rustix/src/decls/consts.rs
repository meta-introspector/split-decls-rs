macro_rules! deps {
    () => {
        Opcode!();
    };
}

macro_rules! consts {
    () => {
        deps!();
        # [cfg (any (target_arch = "mips" , target_arch = "mips32r6" , target_arch = "mips64" , target_arch = "mips64r6" , target_arch = "powerpc" , target_arch = "powerpc64" , target_arch = "sparc" , target_arch = "sparc64"))] mod consts { use super :: Opcode ; pub (super) const NONE : Opcode = 1 ; pub (super) const READ : Opcode = 2 ; pub (super) const WRITE : Opcode = 4 ; pub (super) const SIZE_BITS : Opcode = 13 ; pub (super) const DIR_BITS : Opcode = 3 ; }
    };
}

consts!()