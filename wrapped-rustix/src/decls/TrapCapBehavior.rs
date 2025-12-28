macro_rules! TrapCapBehavior {
    () => {
        # [doc = " `PROC_TRAPCAP_CTL_*`"] # [derive (Copy , Clone , Debug , Eq , PartialEq)] # [repr (i32)] pub enum TrapCapBehavior { # [doc = " Disable the [`Signal::Trap`] signal delivery on capability mode access"] # [doc = " violations."] Disable = PROC_TRAPCAP_CTL_DISABLE , # [doc = " Enable the [`Signal::Trap`] signal delivery on capability mode access"] # [doc = " violations."] Enable = PROC_TRAPCAP_CTL_ENABLE , }
    };
}

TrapCapBehavior!();