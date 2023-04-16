use crate::fake::{self, SyscallDriver};
use fake::rng::*;
use libtock_platform::{
    share::{self},
    AllowRw, DefaultConfig, Subscribe, YieldNoWaitReturn,
};

//Test the command implementation
#[test]
fn command() {
    let rng = Rng::new();
    assert!(rng.command(EXISTS, 0, 0).is_success());
    assert!(rng.command(ASK_FOR_RANDOM_BYTES, 7, 0).is_success());
    assert!(rng.command(ASK_FOR_RANDOM_BYTES, 5, 0).is_success());

    assert!(rng
        .allow_readwrite(RW_ALLOW, RwAllowBuffer::default())
        .is_ok());
    assert!(rng.allow_readwrite(1, RwAllowBuffer::default()).is_err());
}

// Integration test that verifies Rng works with fake::Kernel and
// libtock_platform::Syscalls.
#[test]
fn kernel_integration() {
    use libtock_platform::Syscalls;
    let kernel = fake::Kernel::new();
    let rng = Rng::new();
    kernel.add_driver(&rng);
    assert!(fake::Syscalls::command(DRIVER_NUM, EXISTS, 1, 2).is_success());
    let mut buf: [u8; 10] = [0; 10];
    let mut buf_sync: [u8; 3] = [0; 3];
    let listener: Cell<Option<(u32, u32)>> = Cell::new(None);
    share::scope::<
        (
            AllowRw<_, DRIVER_NUM, RW_ALLOW>,
            Subscribe<_, DRIVER_NUM, 0>,
        ),
        _,
        _,
    >(|handle| {
        let (allow_rw, subscribe) = handle.split();
        assert!(
            fake::Syscalls::allow_rw::<DefaultConfig, DRIVER_NUM, RW_ALLOW>(allow_rw, &mut buf)
                .is_ok()
        );
        assert!(
            fake::Syscalls::subscribe::<_, _, DefaultConfig, DRIVER_NUM, 0>(subscribe, &listener)
                .is_ok()
        );

        assert!(fake::Syscalls::command(DRIVER_NUM, ASK_FOR_RANDOM_BYTES, 5, 0).is_success());

        rng.add_bytes(&[1, 2, 3]);

        assert_eq!(fake::Syscalls::yield_no_wait(), YieldNoWaitReturn::NoUpcall);

        rng.add_bytes(&[4, 5, 6]);

        assert_eq!(fake::Syscalls::yield_no_wait(), YieldNoWaitReturn::Upcall);

        assert_eq!(listener.get(), Some((0, 5)));

        assert!(
            fake::Syscalls::allow_rw::<DefaultConfig, DRIVER_NUM, RW_ALLOW>(
                allow_rw,
                &mut buf_sync
            )
            .is_ok()
        );
        rng.add_bytes_sync(&[6, 7, 8, 9]);
        assert!(fake::Syscalls::command(DRIVER_NUM, ASK_FOR_RANDOM_BYTES, 5, 0).is_success());
        assert_eq!(fake::Syscalls::yield_no_wait(), YieldNoWaitReturn::Upcall);

        //size of the buffer is smaller than number of random bytes requested
        assert_eq!(listener.get(), Some((0, 3)));
    });

    assert!(buf[..5].eq(&[1, 2, 3, 4, 5]));
    assert!(buf_sync.eq(&[6, 7, 8]));
}
