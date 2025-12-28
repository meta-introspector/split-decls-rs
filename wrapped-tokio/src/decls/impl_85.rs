macro_rules! impl_85 {
    () => {
        impl < Fut : Future > Future for MaybeDone < Fut > { type Output = () ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let output = match self . as_mut () . project () { MaybeDoneProj :: Future { future } => ready ! (future . poll (cx)) , MaybeDoneProj :: Done { .. } => return Poll :: Ready (()) , MaybeDoneProj :: Gone => panic ! ("MaybeDone polled after value taken") , } ; self . set (MaybeDone :: Done { output }) ; Poll :: Ready (()) } }
    };
}

impl_85!()