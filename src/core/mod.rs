//! The core module contains all common data structures
//! used throughout the system to manage the window manager's
//! internal state. Basically, everything that is independent
//! of configs or the actual windowing itself.

mod rectangle;
mod stack;
mod workspace;

pub use core::rectangle::Rectangle;
pub use core::stack::Stack;
pub use core::workspace::Workspace;
