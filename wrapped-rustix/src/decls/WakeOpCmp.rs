macro_rules! WakeOpCmp {
    () => {
        # [doc = " `FUTEX_OP_CMP_*` operations for use with [`wake_op`]."] # [derive (Debug , Copy , Clone , Eq , PartialEq)] # [repr (u32)] pub enum WakeOpCmp { # [doc = " `FUTEX_OP_CMP_EQ`: `if oldval == cmparg { wake(); }`"] Eq = 0 , # [doc = " `FUTEX_OP_CMP_EQ`: `if oldval != cmparg { wake(); }`"] Ne = 1 , # [doc = " `FUTEX_OP_CMP_EQ`: `if oldval < cmparg { wake(); }`"] Lt = 2 , # [doc = " `FUTEX_OP_CMP_EQ`: `if oldval <= cmparg { wake(); }`"] Le = 3 , # [doc = " `FUTEX_OP_CMP_EQ`: `if oldval > cmparg { wake(); }`"] Gt = 4 , # [doc = " `FUTEX_OP_CMP_EQ`: `if oldval >= cmparg { wake(); }`"] Ge = 5 , }
    };
}

WakeOpCmp!();