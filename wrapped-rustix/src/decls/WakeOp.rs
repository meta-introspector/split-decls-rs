macro_rules! WakeOp {
    () => {
        # [doc = " `FUTEX_OP_*` operations for use with [`wake_op`]."] # [derive (Debug , Copy , Clone , Eq , PartialEq)] # [repr (u32)] # [allow (clippy :: identity_op)] pub enum WakeOp { # [doc = " `FUTEX_OP_SET`: `uaddr2 = oparg;`"] Set = 0 , # [doc = " `FUTEX_OP_ADD`: `uaddr2 += oparg;`"] Add = 1 , # [doc = " `FUTEX_OP_OR`: `uaddr2 |= oparg;`"] Or = 2 , # [doc = " `FUTEX_OP_ANDN`: `uaddr2 &= ~oparg;`"] AndN = 3 , # [doc = " `FUTEX_OP_XOR`: `uaddr2 ^= oparg;`"] XOr = 4 , # [doc = " `FUTEX_OP_SET | FUTEX_OP_ARG_SHIFT`: `uaddr2 = (oparg << 1);`"] SetShift = 0 | 8 , # [doc = " `FUTEX_OP_ADD | FUTEX_OP_ARG_SHIFT`: `uaddr2 += (oparg << 1);`"] AddShift = 1 | 8 , # [doc = " `FUTEX_OP_OR | FUTEX_OP_ARG_SHIFT`: `uaddr2 |= (oparg << 1);`"] OrShift = 2 | 8 , # [doc = " `FUTEX_OP_ANDN | FUTEX_OP_ARG_SHIFT`: `uaddr2 &= !(oparg << 1);`"] AndNShift = 3 | 8 , # [doc = " `FUTEX_OP_XOR | FUTEX_OP_ARG_SHIFT`: `uaddr2 ^= (oparg << 1);`"] XOrShift = 4 | 8 , }
    };
}

WakeOp!()