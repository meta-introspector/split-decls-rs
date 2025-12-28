macro_rules! other_400 {
    () => {
        # [allow (missing_docs)] # [repr (C)] # [derive (Copy , Clone)] pub union ioprio_union { pub recv_flags : IoringRecvFlags , pub send_flags : IoringSendFlags , pub accept_flags : IoringAcceptFlags , pub ioprio : u16 , }
    };
}

other_400!();