macro_rules! deps {
    () => {
        BlockOnTransferredOwner!();
    };
}

macro_rules! BlockTransferredResult {
    () => {
        deps!();
        pub (crate) enum BlockTransferredResult < 'me > { # [doc = " The current thread is the owner of the transferred query"] # [doc = " and it can claim it if it wants to."] ImTheOwner , # [doc = " The query is owned/running on another thread."] OwnedBy (Box < BlockOnTransferredOwner < 'me > >) , # [doc = " The query has transferred its ownership to another query previously but that query has"] # [doc = " since then completed and released the lock."] Released , }
    };
}

BlockTransferredResult!()