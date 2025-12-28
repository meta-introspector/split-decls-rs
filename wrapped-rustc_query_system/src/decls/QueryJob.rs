macro_rules! deps {
    () => {
        QueryJobId!();
        QueryLatch!();
    };
}

macro_rules! QueryJob {
    () => {
        deps!();
        # [doc = " Represents an active query job."] # [derive (Debug)] pub struct QueryJob < I > { pub id : QueryJobId , # [doc = " The span corresponding to the reason for which this query was required."] pub span : Span , # [doc = " The parent query job which created this job and is implicitly waiting on it."] pub parent : Option < QueryJobId > , # [doc = " The latch that is used to wait on this job."] latch : Option < QueryLatch < I > > , }
    };
}

QueryJob!()