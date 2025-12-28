macro_rules! deps {
    () => {
        Cpuid!();
        Result!();
    };
}

macro_rules! membarrier_cpu {
    () => {
        deps!();
        # [doc = " `membarrier(cmd, MEMBARRIER_CMD_FLAG_CPU, cpu)`—Perform a memory barrier"] # [doc = " with a specific CPU."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/membarrier.2.html"] # [inline] pub fn membarrier_cpu (cmd : MembarrierCommand , cpu : Cpuid) -> io :: Result < () > { backend :: thread :: syscalls :: membarrier_cpu (cmd , cpu) }
    };
}

membarrier_cpu!()