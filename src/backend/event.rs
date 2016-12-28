//! Events are used so that backends (X11, XCB, Wayland) can
//! report back to the window manager if something changed, e.g.
//! new windows got created, a window got closed or if
//! some user input happened.

use core::Rectangle;

/// A cross-section of all events that can be generated/handled
/// by xlib, xcb and wayland.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Event<Window> {
    /// Something on the backend changed, for example
    /// screen got resized or xrandr layout got changed.
    BackendChanged,
    /// A new window has been created
    WindowCreated(Window),
    /// A window has been closed/killed
    WindowClosed(Window),
    /// A window has been hid, but is still around
    WindowHid(Window),
    /// A windowhas been revealed from hidden status
    WindowRevealed(Window),
    /// A window/app is requesting a change in size
    WindowChangeRequest(Window, Rectangle),
    /// The mouse pointer has entered a window's
    /// frame
    MouseEnter(Option<Window>),
    /// The mouse pointer has left a window's frame
    MouseLeave(Option<Window>),
    /// A button has been pressed
    ButtonPressed(Window, Option<Window>),
    /// A button has been released
    ButtonReleased,
    /// A key has been pressed
    KeyPressed(Window),
    /// An unknown or not important event
    Unknown,
}
