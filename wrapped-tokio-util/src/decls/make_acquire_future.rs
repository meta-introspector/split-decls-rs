macro_rules! deps {
    () => {
        PollSendError!();
    };
}

macro_rules! make_acquire_future {
    () => {
        deps!();
        async fn make_acquire_future < T > (data : Option < Sender < T > > ,) -> Result < OwnedPermit < T > , PollSendError < T > > { match data { Some (sender) => sender . reserve_owned () . await . map_err (| _ | PollSendError (None)) , None => unreachable ! ("this future should not be pollable in this state") , } }
    };
}

make_acquire_future!()