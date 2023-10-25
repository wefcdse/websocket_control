//! a crate which aims to control Computer Craft computers
//! via websocket.
//!
//!
//!

pub use get_router_with_tick_func::serve_tick_func;
mod get_router_with_tick_func;

use get_router::get_router;
mod get_router;

use socket_collection::{SocketCollection, SocketCollectionHandle, SocketCollectionStateHandle};
mod socket_collection;

pub use ports::Ports;
mod ports;

pub use port::Port;
mod port;

pub use error::{Errors, ToErrorsResult};
mod error;

pub use support_type::{ColorId, Event, PeripheralType, Side};
mod support_type;

mod port_functions;

pub mod utils;
