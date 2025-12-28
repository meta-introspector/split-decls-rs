macro_rules! memory_allocation {
    () => {
        # [doc = " Memory-allocation functions are out of scope for rustix."] # [doc = ""] # [doc = " It is possible to implement `malloc`, `free`, and similar functions in"] # [doc = " Rust, however rustix itself is focused on syscall-like functions. This"] # [doc = " module contains an incomplete list of such functions."] # [doc = ""] # [doc = " There are several allocator implementations for Rust; one of them is"] # [doc = " [dlmalloc]. For a rustix-based implementation, see [rustix-dlmalloc]."] # [doc = " Another allocator implementation is [talc]."] # [doc = ""] # [doc = " [dlmalloc]: https://crates.io/crates/dlmalloc"] # [doc = " [talc]: https://crates.io/crates/talc"] # [doc = " [rustix-dlmalloc]: https://crates.io/crates/rustix-dlmalloc"] pub mod memory_allocation { not_implemented ! (malloc) ; not_implemented ! (realloc) ; not_implemented ! (calloc) ; not_implemented ! (free) ; not_implemented ! (posix_memalign) ; not_implemented ! (aligned_alloc) ; not_implemented ! (malloc_usable_size) ; }
    };
}

memory_allocation!();