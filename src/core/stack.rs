use std::iter::FromIterator;
use std::fmt::Debug;

/// Handles focus tracking on a workspace.
/// `focus` keeps track of the focused window's id
/// and `up` and `down` are the windows above or
/// below the focus stack respectively.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Stack<T> {
    pub focus: T,
    pub up: Vec<T>,
    pub down: Vec<T>,
}

impl<T: Debug + Copy + Clone + Eq> Stack<T> {
    /// Create a new stack with the given values
    pub fn new(f: T, up: Vec<T>, down: Vec<T>) -> Stack<T> {
        trace!("creating new stack from {:?}/{:?}/{:?}", up, f, down);
        Stack {
            focus: f,
            up: up,
            down: down,
        }
    }

    /// Add a new element to the stack
    /// and automatically focus it.
    pub fn add(&self, t: T) -> Stack<T> {
        trace!("adding {:?} to stack", t);
        Stack {
            focus: t,
            up: self.up.clone(),
            down: self.down
                .clone()
                .into_iter()
                .chain((vec![self.focus]).into_iter())
                .collect(),
        }
    }

    /// Flatten the stack into a list
    pub fn integrate<C: FromIterator<T>>(&self) -> C {
        trace!("integrating stack");
        self.up
            .iter()
            .rev()
            .chain(vec![self.focus].iter())
            .chain(self.down.iter())
            .cloned()
            .collect()
    }

    /// Filter the stack to retain only windows
    /// that yield true in the given filter function
    pub fn filter<F>(&self, f: F) -> Option<Stack<T>>
        where F: Fn(&T) -> bool
    {
        trace!("filtering stack");
        let lrs: Vec<T> = (vec![self.focus])
            .iter()
            .chain(self.down.iter())
            .filter(|&x| f(x))
            .cloned()
            .collect();

        if !lrs.is_empty() {
            let first = lrs[0];
            let rest: Vec<T> = lrs.iter().skip(1).cloned().collect();
            let filtered: Vec<T> = self.up
                .iter()
                .filter(|&x| f(x))
                .cloned()
                .collect();
            let stack: Stack<T> = Stack::<T>::new(first, filtered, rest);

            trace!("stack after filtering non-empty");
            Some(stack)
        } else {
            let filtered: Vec<T> = self.up.clone().into_iter().filter(|x| f(x)).collect();
            if !filtered.is_empty() {
                let first = filtered[0];
                let rest: Vec<T> = filtered.iter().skip(1).cloned().collect();
                trace!("stack after filtering non-empty");
                Some(Stack::<T>::new(first, rest, Vec::new()))
            } else {
                trace!("stack after filtering empty");
                None
            }
        }
    }

    /// Move the focus to the next element in the `up` list
    pub fn focus_up(&self) -> Stack<T> {
        trace!("focusing up in stack");
        if self.up.is_empty() {
            let tmp: Vec<T> = (vec![self.focus])
                .into_iter()
                .chain(self.down.clone().into_iter())
                .rev()
                .collect();
            let xs: Vec<T> = tmp.iter()
                .skip(1)
                .cloned()
                .collect();

            Stack::<T>::new(tmp[0], xs, Vec::new())
        } else {
            let down = (vec![self.focus])
                .into_iter()
                .chain(self.down.clone().into_iter())
                .collect();
            let up = self.up.iter().skip(1).cloned().collect();
            Stack::<T>::new(self.up[0], up, down)
        }
    }

    /// Move the focus down
    pub fn focus_down(&self) -> Stack<T> {
        trace!("focusing down in stack. reversing, focussing up, reversing back");
        self.reverse().focus_up().reverse()
    }

    pub fn swap_up(&self) -> Stack<T> {
        trace!("swapping up in stack");
        if self.up.is_empty() {
            Stack::<T>::new(self.focus,
                            self.down.iter().rev().cloned().collect(),
                            Vec::new())
        } else {
            let x = self.up[0];
            let xs = self.up.iter().skip(1).cloned().collect();
            let rs = (vec![x]).into_iter().chain(self.down.clone().into_iter()).collect();
            Stack::<T>::new(self.focus, xs, rs)
        }
    }

    pub fn swap_down(&self) -> Stack<T> {
        trace!("swapping down in stack. reversing, swapping up, reversing back");
        self.reverse().swap_up().reverse()
    }

    pub fn swap_master(&self) -> Stack<T> {
        trace!("swapping stack to master");
        if self.up.is_empty() {
            return self.clone();
        }

        let r: Vec<T> = self.up
            .iter()
            .rev()
            .cloned()
            .collect();
        let x = r[0];
        let xs: Vec<T> = r.iter()
            .skip(1)
            .cloned()
            .collect();
        let rs: Vec<T> = xs.into_iter()
            .chain((vec![x]).into_iter())
            .chain(self.down.clone().into_iter())
            .collect();

        Stack::<T>::new(self.focus, Vec::new(), rs)
    }

    /// Reverse the stack by exchanging
    /// the `up` and `down` lists
    pub fn reverse(&self) -> Stack<T> {
        trace!("reversing stack {:?}/{:?}/{:?}",
               self.up,
               self.focus,
               self.down);
        Stack::<T>::new(self.focus, self.down.clone(), self.up.clone())
    }

    /// Return the number of elements tracked by the stack
    pub fn len(&self) -> usize {
        1 + self.up.len() + self.down.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Checks if the given window is tracked by the stack
    pub fn contains(&self, window: T) -> bool {
        trace!("checking if stack contains {:?}", window);
        self.focus == window || self.up.contains(&window) || self.down.contains(&window)
    }
}

impl<T: Debug> From<T> for Stack<T> {
    /// Create a new stack with only the given element
    /// as the focused one and initialize the rest to empty.
    fn from(t: T) -> Stack<T> {
        trace!("creating new stack from {:?}", t);
        Stack {
            focus: t,
            up: Vec::new(),
            down: Vec::new(),
        }
    }
}
