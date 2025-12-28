macro_rules! PollN {
    () => {
        # [allow (missing_docs)] pub struct PollN < T , E > { and_return : Option < Result < T , E > > , finish_at : usize , polls : usize , }
    };
}

PollN!()