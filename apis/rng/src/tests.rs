use super::*;
use core::cell::Cell;
use libtock_platform::{share, AllowRw, ErrorCode, Subscribe, YieldNoWaitReturn};
use libtock_unittest::fake;

type Rng = super::Rng<fake::Syscalls>;

#[test]
fn no_driver() {
    let _kernel = fake::Kernel::new();
    assert_eq!(Rng::exists(), Err(ErrorCode::NoDevice));
}

#[test]
fn driver_check() {
    let kernel = fake::Kernel::new();
    let driver = fake::Rng::new();
    kernel.add_driver(&driver);

    assert_eq!(Rng::exists(), Ok(()));
}

#[test]
fn get_random() {
    let kernel = fake::Kernel::new();
    let driver = fake::Rng::new();
    kernel.add_driver(&driver);

    let mut buf: [u8; 10] = [0; 10];
    let listener: Cell<Option<(u32, u32)>> = Cell::new(None);

    share::scope::<
        (
            AllowRw<_, DRIVER_NUM, RW_ALLOW>,
            Subscribe<_, DRIVER_NUM, 0>,
        ),
        _,
        _,
    >(
        |handle: share::Handle<(
            AllowRw<fake::Syscalls, DRIVER_NUM, RW_ALLOW>,
            Subscribe<fake::Syscalls, DRIVER_NUM, 0>,
        )>| {
            let (allow_rw, subscribe) = handle.split();

            assert!(Rng::set_buffer(&mut buf, allow_rw).is_ok());
            assert!(Rng::register_listener(&listener, subscribe).is_ok());

            assert!(Rng::get_random(5).is_ok());

            driver.add_bytes(&[1, 2, 3]);

            assert_eq!(fake::Syscalls::yield_no_wait(), YieldNoWaitReturn::NoUpcall);

            driver.add_bytes(&[4, 5, 6]);

            assert_eq!(fake::Syscalls::yield_no_wait(), YieldNoWaitReturn::Upcall);

            assert_eq!(listener.get(), Some((0, 5)));
        },
    );
    assert!(buf[..5].eq(&[1, 2, 3, 4, 5]));
}
#[test]
fn get_sync() {
    let kernel = fake::Kernel::new();
    let driver = fake::Rng::new();
    kernel.add_driver(&driver);

    driver.add_bytes_sync(&[10, 20, 30, 40, 50]);

    let mut buf: [u8; 10] = [0; 10];

    assert_eq!(Rng::get_random_sync(&mut buf, 5), Ok(5));
    assert!(buf[..5].eq(&[10, 20, 30, 40, 50]));

    driver.add_bytes_sync(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);

    assert_eq!(Rng::get_random_sync(&mut buf, 11), Ok(10));
}
