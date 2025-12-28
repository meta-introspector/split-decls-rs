macro_rules! msg {
    () => {
        # [cfg (not (any (windows , target_os = "espidf" , target_os = "horizon" , target_os = "vita")))] mod msg ;
    };
}

msg!();