pub use self::handle::{Handle,Request};
pub use self::response::{Headers,Response};

pub mod body;
pub mod handle;
pub mod header;
pub mod response;

#[inline]
pub fn handle() -> Handle {
    Handle::new()
}
