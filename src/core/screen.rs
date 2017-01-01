use core::workspace::Workspace;
use core::stack::Stack;
use std::fmt::Debug;

/// The screen structure holds all information about a *visible*
/// screen. A workspace manages the contents of a single workspace itself,
/// shown or hidden. A screen always represents a *visible* workspace.
///
/// A screen is represented by the workspace it manages
/// and an ID for the screen it is being shown on.
pub struct Screen<Window> {
    pub workspace: Workspace<Window>,
    pub screen_id: u32,
}

impl<Window: Clone> Clone for Screen<Window> {
    fn clone(&self) -> Screen<Window> {
        Screen {
            workspace: self.workspace.clone(),
            screen_id: self.screen_id,
        }
    }
}

impl<Window: Copy + Clone + PartialEq + Eq + Debug> Screen<Window> {
    /// Create a new screen for the given workspace
    /// and the given dimensions
    ///
    /// # Examples
    ///
    /// ```
    /// # use sabiwm::core::{Screen, Workspace};
    /// let workspace : Workspace<u32> = Workspace::new(0, "foo", None);
    /// let screen = Screen::new(workspace, 2);
    /// ```
    ///
    /// # Arguments
    /// `workspace` - The [`Workspace`] the screen manages
    /// `screen_id` - The global identifier for this screen
    ///
    /// # Return value
    /// A new [`Screen`] managing the given [`Workspace`]
    ///
    /// [`Screen`]: struct.Screen.html
    /// [`Workspace`]: struct.Workspace.html
    pub fn new(workspace: Workspace<Window>, screen_id: u32) -> Screen<Window> {
        Screen {
            workspace: workspace,
            screen_id: screen_id,
        }
    }

    /// Checks if the screen's workspace contains
    /// the given window
    ///
    /// # Arguments
    /// `window` - The window to check for
    ///
    /// # Return value
    /// `true` if the screen contains the given window
    ///
    /// [`Screen`]: struct.Screen.html
    /// [`Workspace`]: struct.Workspace.html
    pub fn contains(&self, window: Window) -> bool {
        self.workspace.contains(window)
    }

    /// Returns the number of windows in the
    /// [`Screen`]'s [`Workspace`]
    ///
    /// # Return value
    /// Number of window visible in the current screen
    ///
    /// [`Screen`]: struct.Screen.html
    /// [`Workspace`]: struct.Workspace.html
    pub fn len(&self) -> usize {
        self.workspace.len()
    }

    /// Returns a list of all windows visible on
    /// the [`Screen`]'s [`Workspace`]
    ///
    /// # Return value
    /// List of all windows on this [`Screen`]
    ///
    /// [`Screen`]: struct.Screen.html
    /// [`Workspace`]: struct.Workspace.html
    pub fn windows(&self) -> Vec<Window> {
        self.workspace.windows()
    }

    /// Map a given function over the contained [`Workspace`]
    ///
    /// # Arguments
    /// `f` - A function modifying a [`Workspace`]
    ///
    /// # Return value
    /// A new [`Screen`] with a modified [`Workspace`]
    ///
    /// [`Screen`]: struct.Screen.html
    /// [`Workspace`]: struct.Workspace.html
    pub fn map_workspace<F>(&self, f: F) -> Screen<Window>
        where F: Fn(Workspace<Window>) -> Workspace<Window>
    {
        let workspace = f(self.workspace.clone());
        Screen::new(workspace, self.screen_id)
    }

    /// Map a given function over the contained [`Stack`]
    ///
    /// # Arguments
    /// `f` - A function modifying a [`Stack`]
    ///
    /// # Return value
    /// A new [`Screen`] with a modified [`Stack`]
    ///
    /// [`Screen`]: struct.Screen.html
    /// [`Stack`]: struct.Stack.html
    pub fn map<F>(&self, f: F) -> Screen<Window>
        where F: Fn(Stack<Window>) -> Stack<Window>
    {
        Screen::new(self.workspace.map(f), self.screen_id)
    }

    /// Map a given function over the contained [`Stack`].
    /// The [`Stack`] might end up empty or discarded,
    /// in which case the function shall return `None`
    ///
    /// # Arguments
    /// `f` - A function modifying a [`Stack`]
    ///
    /// # Return value
    /// A new [`Screen`] with a modified [`Stack`]
    ///
    /// [`Screen`]: struct.Screen.html
    /// [`Stack`]: struct.Stack.html
    pub fn map_option<F>(&self, f: F) -> Screen<Window>
        where F: Fn(Stack<Window>) -> Option<Stack<Window>>
    {
        Screen::new(self.workspace.map_option(f), self.screen_id)
    }

    /// Map a given function over the contained [`Stack`].
    /// The [`Stack`] might end up empty or discarded,
    /// in which case the function shall return `None`.
    /// In that case, the [`Stack`] is replaced with the supplied
    /// default value.
    ///
    /// # Arguments
    /// `default` - The default value to use in case `f` returns `None`
    /// `f` - A function modifying a [`Stack`]
    ///
    /// # Return value
    /// A new [`Screen`] with a modified [`Stack`]
    ///
    /// [`Screen`]: struct.Screen.html
    /// [`Stack`]: struct.Stack.html
    pub fn map_or<F>(&self, default: Stack<Window>, f: F) -> Screen<Window>
        where F: Fn(Stack<Window>) -> Stack<Window>
    {
        Screen::new(self.workspace.map_or(default, f), self.screen_id)
    }
}
