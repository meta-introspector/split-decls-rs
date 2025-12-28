macro_rules! Heap {
    () => {
        # [doc = "  heap used to build the Huffman trees"] # [doc = ""] # [doc = " The sons of heap[n] are heap[2*n] and heap[2*n+1]. heap[0] is not used."] # [doc = " The same heap array is used to build all trees."] # [derive (Clone)] struct Heap { heap : [u32 ; 2 * L_CODES + 1] , # [doc = " number of elements in the heap"] heap_len : usize , # [doc = " element of the largest frequency"] heap_max : usize , depth : [u8 ; 2 * L_CODES + 1] , }
    };
}

Heap!()