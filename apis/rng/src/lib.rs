#![no_std]

use core::cell::Cell;

use libtock_platform::{share, AllowRw, DefaultConfig, ErrorCode, Subscribe, Syscalls};
pub struct Rng<S: Syscalls>(S);

impl<S: Syscalls> Rng<S> {
    pub fn exists() -> Result<(), ErrorCode> {
        S::command(DRIVER_NUM, EXISTS, 0, 0).to_result()
    }

    /// Register a listener to be called when the random generation is finished
    pub fn register_listener<'share>(
        listener: &'share Cell<Option<(u32, u32)>>,
        subscribe: share::Handle<Subscribe<'share, S, DRIVER_NUM, 0>>,
    ) -> Result<(), ErrorCode> {
        S::subscribe::<_, _, DefaultConfig, DRIVER_NUM, 0>(subscribe, listener)
    }

    /// Sets the buffer in which the random numbers will be written
    pub fn set_buffer<'share>(
        buffer: &'share mut [u8],
        allow_rw: share::Handle<AllowRw<'share, S, DRIVER_NUM, 0>>,
    ) -> Result<(), ErrorCode> {
        S::allow_rw::<DefaultConfig, DRIVER_NUM, 0>(allow_rw, buffer)
    }

    /// Initiates an async random generation for n bytes
    /// A buffer and a callback should have been set before
    pub fn get_random(n: u32) -> Result<(), ErrorCode> {
        S::command(DRIVER_NUM, ASK_FOR_RANDOM_BYTES, n, 0).to_result()
    }

    ///Initiates a synchronous random number generation
    /// `n` random bytes will be written in `buf`
    /// returns the number of bytes successfully written or error
    pub fn get_random_sync(buf: &mut [u8], n: u32) -> Result<u32, ErrorCode> {
        let listener: Cell<Option<(u32, u32)>> = Cell::new(None);
        share::scope::<(AllowRw<_, DRIVER_NUM, 0>, Subscribe<_, DRIVER_NUM, 0>), _, _>(|handle| {
            let (allow_rw, subscribe) = handle.split();

            if let Ok(()) = Self::set_buffer(buf, allow_rw) {
                if let Ok(()) = Self::register_listener(&listener, subscribe) {
                    if let Ok(()) = Self::get_random(n) {
                        while listener.get() == None {
                            S::yield_wait();
                        }
                    }
                }
            }
        });

        match listener.get() {
            Some((_, bytes_received)) => Ok(bytes_received),
            None => Err(ErrorCode::Fail),
        }
    }
}

// -----------------------------------------------------------------------------
// Driver number and command IDs
// -----------------------------------------------------------------------------

const DRIVER_NUM: u32 = 0x40001;

// Command IDs

const EXISTS: u32 = 0;
const ASK_FOR_RANDOM_BYTES: u32 = 1;
