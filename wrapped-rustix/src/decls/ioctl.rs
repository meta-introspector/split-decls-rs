macro_rules! ioctl {
    () => {
        # [cfg (linux_kernel)] mod ioctl ;
    };
}

ioctl!()