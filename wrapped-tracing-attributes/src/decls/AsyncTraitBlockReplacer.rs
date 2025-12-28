macro_rules! AsyncTraitBlockReplacer {
    () => {
        struct AsyncTraitBlockReplacer < 'a > { block : & 'a Block , patched_block : Block , }
    };
}

AsyncTraitBlockReplacer!();