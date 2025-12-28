macro_rules! deps {
    () => {
        Thread!();
        ThreadLocal!();
    };
}

macro_rules! ThreadIdManager {
    () => {
        deps!();
        # [doc = " Thread ID manager which allocates thread IDs. It attempts to aggressively"] # [doc = " reuse thread IDs where possible to avoid cases where a ThreadLocal grows"] # [doc = " indefinitely when it is used by many short-lived threads."] struct ThreadIdManager { free_from : usize , free_list : Option < BinaryHeap < Reverse < usize > > > , }
    };
}

ThreadIdManager!();