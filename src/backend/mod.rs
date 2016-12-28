mod event;
mod xcb;

pub use backend::event::Event;
pub use backend::xcb::Xcb;

// use backend::Event;
use core::Rectangle;
use errors::*;

/// A general trait for all backends (X11, XCB, Wayland)
pub trait Backend {
    type Window;

    /// Create a new instance of the Backend.
    ///
    /// # Return value
    ///
    /// A new [`Backend`]
    ///
    /// [`Backend`]: trait.Backend.html
    fn new() -> Result<Self> where Self: ::std::marker::Sized;
    /// Determines if the window represented by the given ID
    /// is a dock and should be ignored for layouts, etc.
    ///
    /// # Arguments
    /// `window` - the [`Window`] ID
    ///
    /// # Return value
    ///
    /// True if the given [`Window`] is a dock, false otherwise
    ///
    /// [`Window`]: trait.Backend.html#associatedtype.Window
    fn is_dock(&self, window: Self::Window) -> bool;
    fn is_window(&self, window: Self::Window) -> bool;
    /// Returns a vector of all screens currently handled by the
    /// window manager.
    ///
    /// # Return value
    ///
    /// A vector of [`Rectangle`]s, each representing
    /// a screen, its position and its dimensions
    ///
    /// [`Rectangle`]: ../core/struct.Rectangle.html
    fn screens(&self) -> Vec<Rectangle>;
    /// Returns the number of screens. Basically just a shorthand for
    /// ```
    /// self.screens.len()
    /// ```
    ///
    /// # Return value
    ///
    /// Number of screens currently handled by the window manager
    fn number_of_screens(&self) -> usize;
    /// Returns the name/title of the given [`Window`]
    ///
    /// # Arguments
    /// `window` - the [`Window`] ID to get the title of
    ///
    /// # Return value
    ///
    /// A string representing the [`Window`]'s title.
    ///
    /// [`Window`]: trait.Backend.html#associatedtype.Window
    fn window_name(&self, window: Self::Window) -> String;
    /// Returns the given [`Window`]s class name.
    /// Useful for custom mappings, e.g. always move `mpv` to
    /// workspace 4.
    ///
    /// # Arguments
    /// `window` - the [`Window`] ID
    ///
    /// # Return value
    ///
    /// A string representing the [`Window`]'s class
    ///
    /// [`Window`]: trait.Backend.html#associatedtype.Window
    fn class_name(&self, window: Self::Window) -> String;
    /// Returns a vector of all [`Window`] IDs currently handled
    /// by the window manager's backend.
    ///
    /// # Return value
    ///
    /// A vector of all [`Window`] IDs
    ///
    /// [`Window`]: trait.Backend.html#associatedtype.Window
    fn windows(&self) -> Result<Vec<Self::Window>>;
    /// Tells the backend to resize the given [`Window`] to the
    /// given `width` and `height`.
    ///
    /// # Arguments
    /// `window` - the [`Window`] ID
    /// `width` - new width to resize to
    /// `height` - new height to resize to
    ///
    /// [`Window`]: trait.Backend.html#associatedtype.Window
    fn resize_window(&self, window: Self::Window, width: u32, height: u32);
    /// Tells the backend to move the [`Window`] to the given location
    ///
    /// # Arguments
    ///
    /// `window` - the [`Window`] ID
    /// `x` - the new x position of the upper left corner
    /// `y` - the new y position of the upper left corner
    ///
    /// [`Window`]: trait.Backend.html#associatedtype.Window
    fn move_window(&self, window: Self::Window, x: u32, y: u32);
    /// Shows/reveals the window if it has previously been hidden
    /// and notifies it about the event.
    ///
    /// # Arguments
    ///
    /// `window` - the [`Window`] ID
    ///
    /// [`Window`]: trait.Backend.html#associatedtype.Window
    fn show_window(&self, window: Self::Window);
    /// Hides the window if it has previously been shown
    /// and notifies it about the event.
    ///
    /// # Arguments
    ///
    /// `window` - the [`Window`] ID
    ///
    /// [`Window`]: trait.Backend.html#associatedtype.Window
    fn hide_window(&self, window: Self::Window);
    /// Focusses the window, so it is ready to accept direct input
    ///
    /// # Arguments
    ///
    /// `window` - the [`Window`] ID
    ///
    /// [`Window`]: trait.Backend.html#associatedtype.Window
    fn focus_window(&self, window: Self::Window);
    /// Blocks until an event can be provided by the backend.
    /// Does not need to be asynchronous, because as long
    /// as there is no event, the window manager does not need
    /// to do anything.
    ///
    /// # Return value
    ///
    /// An instance of the [`Event`] enum
    ///
    /// [`Event`]: enum.Event.html
    fn event(&self) -> Event<Self::Window>;
}
