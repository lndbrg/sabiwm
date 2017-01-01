// use layout::{Layout, LayoutMessage};
use core::Stack;
use std::fmt::Debug;

/// Represents a single workspace with a `tag` (name),
/// `id`, a `layout` and a `stack` for all windows.
/// A workspace is in charge of all windows belonging
/// to that workspace. At each time, a single screen
/// holds one workspace, while all the other
/// workspaces are hidden in the background, while
/// still being managed.
///
/// # Immutable
///
/// Note that this [`Workspace`] implementation is immutable
/// and that each operation that would modify it, instead
/// returns a new copy of the [`Workspace`] with the modified state.
///
/// [`Workspace`]: struct.Stack.html
pub struct Workspace<Window> {
    ///
    pub id: u32,
    ///
    pub tag: String,
    ///
    pub stack: Option<Stack<Window>>,
}

impl<Window: Clone> Clone for Workspace<Window> {
    fn clone(&self) -> Workspace<Window> {
        Workspace {
            id: self.id,
            tag: self.tag.clone(),
            stack: self.stack.clone(),
        }
    }
}

impl<Window: Copy + Clone + PartialEq + Eq + Debug> Workspace<Window> {
    /// Create a new workspace
    ///
    /// # Examples
    ///
    /// ```
    /// # use sabiwm::core::{Stack, Workspace};
    /// let stack = Stack::from(42u32);
    /// let workspace = Workspace::new(0, "Desktop 0", Some(stack));
    /// assert_eq!(1, workspace.len());
    /// ```
    pub fn new<S: Into<String>>(id: u32,
                                tag: S,
                                stack: Option<Stack<Window>>)
                                -> Workspace<Window> {
        let tag = tag.into();
        trace!("workspace_tag" => tag, "workspace_id" => id; "creating new workspace");
        Workspace {
            id: id,
            tag: tag,
            stack: stack,
        }
    }

    /// Add a new window to the workspace by adding it to the stack.
    /// If the stack doesn't exist yet, create one.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sabiwm::core::Workspace;
    /// let workspace : Workspace<u32> = Workspace::new(0, "Desktop 0", None);
    /// assert_eq!(0, workspace.len());
    /// ```
    pub fn add(&self, window: Window) -> Workspace<Window> {
        trace!("workspace_tag" => self.tag, "workspace_id" => self.id; "adding window {:?} to workspace", window);
        Workspace::new(self.id,
                       self.tag.clone(),
                       Some(self.stack
                           .clone()
                           .map_or(Stack::from(window), |s| s.add(window))))
    }

    /// Remove the given window from the workspace.
    ///
    /// # Arguments
    /// `window` - The window to remove
    ///
    /// # Return value
    /// A new [`Workspace`] without the window
    ///
    /// [`Workspace`]: struct.Workspace.html
    pub fn remove(&self, window: Window) -> Workspace<Window> {
        trace!("workspace_tag" => self.tag, "workspace_id" => self.id; "removing window {:?} from workspace", window);
        Workspace::new(self.id,
                       self.tag.clone(),
                       self.stack.clone().map_or(None, |s| s.filter(|&w| w != window)))
    }

    /// Returns the number of windows contained in this [`Workspace`]
    ///
    /// # Return value
    /// Number of windows in this [`Workspace`]
    /// [`Workspace`]: struct.Workspace.html
    pub fn len(&self) -> usize {
        self.stack.clone().map_or(0, |x| x.len())
    }

    /// Checks if the [`Workspace`] is empty, i.e. if it is not
    /// managing any windows.
    ///
    /// # Return value
    /// `true` if the [`Workspace`] is empty
    /// [`Workspace`]: struct.Workspace.html
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Checks if the workspace contains the given window
    /// [`Workspace`]: struct.Workspace.html
    pub fn contains(&self, window: Window) -> bool {
        trace!("workspace_tag" => self.tag, "workspace_id" => self.id; "checking if workspace contains window {:?}", window);
        self.stack.clone().map_or(false, |x| x.contains(window))
    }

    /// [`Workspace`]: struct.Workspace.html
    pub fn windows(&self) -> Vec<Window> {
        self.stack.clone().map_or(Vec::new(), |s| s.integrate())
    }

    /// [`Workspace`]: struct.Workspace.html
    pub fn peek(&self) -> Option<Window> {
        self.stack.clone().map(|s| s.focus)
    }

    /// [`Workspace`]: struct.Workspace.html
    pub fn map<F>(&self, f: F) -> Workspace<Window>
        where F: Fn(Stack<Window>) -> Stack<Window>
    {
        trace!("workspace_tag" => self.tag, "workspace_id" => self.id; "mapping over workspace");
        Workspace::new(self.id, self.tag.clone(), self.stack.clone().map(f))
    }

    /// [`Workspace`]: struct.Workspace.html
    pub fn map_option<F>(&self, f: F) -> Workspace<Window>
        where F: Fn(Stack<Window>) -> Option<Stack<Window>>
    {
        trace!("workspace_tag" => self.tag, "workspace_id" => self.id; "mapping optional over workspace");
        Workspace::new(self.id,
                       self.tag.clone(),
                       self.stack.clone().map_or(None, f))
    }

    /// [`Workspace`]: struct.Workspace.html
    pub fn map_or<F>(&self, default: Stack<Window>, f: F) -> Workspace<Window>
        where F: Fn(Stack<Window>) -> Stack<Window>
    {
        trace!("workspace_tag" => self.tag, "workspace_id" => self.id; "mapping default over workspace");
        Workspace::new(self.id,
                       self.tag.clone(),
                       Some(self.stack.clone().map_or(default, f)))
    }
}
