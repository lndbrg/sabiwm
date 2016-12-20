// use layout::{Layout, LayoutMessage};
use core::Stack;
use std::fmt::Debug;

/// Represents a single workspace with a `tag` (name),
/// `id`, a `layout` and a `stack` for all windows
pub struct Workspace<Window> {
    pub id: u32,
    pub tag: String,
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

impl<Window: Copy + Clone + Eq + Debug> Workspace<Window> {
    /// Create a new workspace
    ///
    /// # Examples
    ///
    /// ```
    /// # use sabiwm::core::{Stack, Workspace};
    /// let stack = Stack::from(42u32);
    /// let workspace = Workspace::new(0, "Desktop 0".into(), Some(stack));
    /// assert_eq!(1, workspace.len());
    /// ```
    pub fn new(id: u32, tag: String, stack: Option<Stack<Window>>) -> Workspace<Window> {
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
    /// let workspace : Workspace<u32> = Workspace::new(0, "Desktop 0".into(), None);
    /// assert_eq!(0, workspace.len());
    /// ```
    pub fn add(&self, window: Window) -> Workspace<Window> {
        Workspace::new(self.id,
                       self.tag.clone(),
                       Some(self.stack
                           .clone()
                           .map_or(Stack::from(window), |s| s.add(window))))
    }

    /// Returns the number of windows contained in this workspace
    pub fn len(&self) -> usize {
        self.stack.clone().map_or(0, |x| x.len())
    }

    /// Checks if the workspace contains the given window
    pub fn contains(&self, window: Window) -> bool {
        self.stack.clone().map_or(false, |x| x.contains(window))
    }

    pub fn windows(&self) -> Vec<Window> {
        self.stack.clone().map_or(Vec::new(), |s| s.integrate())
    }

    pub fn peek(&self) -> Option<Window> {
        self.stack.clone().map(|s| s.focus)
    }

    pub fn map<F>(&self, f: F) -> Workspace<Window>
        where F: Fn(Stack<Window>) -> Stack<Window>
    {
        Workspace::new(self.id, self.tag.clone(), self.stack.clone().map(|x| f(x)))
    }

    pub fn map_option<F>(&self, f: F) -> Workspace<Window>
        where F: Fn(Stack<Window>) -> Option<Stack<Window>>
    {
        Workspace::new(self.id,
                       self.tag.clone(),
                       self.stack.clone().map_or(None, |x| f(x)))
    }

    pub fn map_or<F>(&self, default: Stack<Window>, f: F) -> Workspace<Window>
        where F: Fn(Stack<Window>) -> Stack<Window>
    {
        Workspace::new(self.id,
                       self.tag.clone(),
                       Some(self.stack.clone().map_or(default, |x| f(x))))
    }
}
