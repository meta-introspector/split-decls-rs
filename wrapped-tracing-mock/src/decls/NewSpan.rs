macro_rules! deps {
    () => {
        MockSubscriber!();
        ExpectedSpan!();
        ExpectedFields!();
        ExpectedAncestry!();
    };
}

macro_rules! NewSpan {
    () => {
        deps!();
        # [doc = " A mock new span."] # [doc = ""] # [doc = " **Note**: This struct contains expectations that can only be asserted"] # [doc = " on when expecting a new span via [`MockSubscriber::new_span`]. They"] # [doc = " cannot be validated on [`MockSubscriber::enter`],"] # [doc = " [`MockSubscriber::exit`], or any other method on [`MockSubscriber`]"] # [doc = " that takes an `ExpectedSpan`."] # [doc = ""] # [doc = " For more details on how to use this struct, see the documentation"] # [doc = " on the [`subscriber`] module."] # [doc = ""] # [doc = " [`subscriber`]: mod@crate::subscriber"] # [doc = " [`MockSubscriber`]: struct@crate::subscriber::MockSubscriber"] # [doc = " [`MockSubscriber::enter`]: fn@crate::subscriber::MockSubscriber::enter"] # [doc = " [`MockSubscriber::exit`]: fn@crate::subscriber::MockSubscriber::exit"] # [doc = " [`MockSubscriber::new_span`]: fn@crate::subscriber::MockSubscriber::new_span"] # [derive (Default , Eq , PartialEq)] pub struct NewSpan { pub (crate) span : ExpectedSpan , pub (crate) fields : ExpectedFields , pub (crate) ancestry : Option < ExpectedAncestry > , }
    };
}

NewSpan!()