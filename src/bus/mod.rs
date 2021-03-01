use core::future::Future;

mod eightbit;
mod fourbit;

pub use self::eightbit::EightBitBus;
pub use self::fourbit::FourBitBus;

use crate::error::Result;

pub trait DataBus {
    type WriteFuture<'a>: Future<Output = Result<()>>;

    fn write<'a>(&'a mut self, byte: u8, data: bool) -> Self::WriteFuture<'a>;

    // TODO
    // fn read(...)
}
