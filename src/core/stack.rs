use std::iter::FromIterator;
use std::fmt::Debug;

/// Handles focus tracking on a workspace.
/// `focus` keeps track of the focused window's id
/// and `up` and `down` are the windows above or
/// below the focus stack respectively.
///
/// # Immutable
///
/// Note that this [`Stack`] implementation is immutable
/// and that each operation that would modify it, instead
/// returns a new copy of the [`Stack`] with the modified state.
///
/// [`Stack`]: struct.Stack.html
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Stack<T> {
    pub focus: T,
    pub up: Vec<T>,
    pub down: Vec<T>,
}

impl<T: Debug + Copy + Clone + Eq> Stack<T> {
    /// Create a new stack with the given values
    ///
    /// # Examples
    ///
    /// ```
    /// # use sabiwm::core::Stack;
    /// let stack = Stack::new(0, Vec::new(), Vec::new());
    /// ```
    ///
    /// # Arguments
    ///
    /// `f` - the focussed element to start with
    /// `up` - vector of all elements up the stack
    /// `down` - vector of all elements down the stack
    ///
    /// # Return value
    ///
    /// A new [`Stack`]
    ///
    /// [`Stack`]: struct.Stack.html
    pub fn new<S: Debug + IntoIterator<Item = T>>(f: T, up: S, down: S) -> Stack<T> {
        trace!("creating new stack from {:?}/{:?}/{:?}", up, f, down);
        Stack {
            focus: f,
            up: up.into_iter().collect(),
            down: down.into_iter().collect(),
        }
    }

    /// Add a new element to the stack
    /// and automatically focus it.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sabiwm::core::Stack;
    /// let stack = Stack::from(42);
    /// let new_stack = stack.add(23);
    ///
    /// assert_eq!(2, new_stack.len());
    /// ```
    ///
    /// # Arguments
    ///
    /// `t` - the new element to add
    ///
    /// # Return value
    ///
    /// The new modified [`Stack`]
    ///
    /// [`Stack`]: struct.Stack.html
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

    /// Flatten the stack into a new container
    ///
    /// # Examples
    ///
    /// ```
    /// # use sabiwm::core::Stack;
    /// let stack = Stack::new(1, vec![2,3], vec![4,5]);
    /// let v = stack.integrate::<Vec<_>>();
    ///
    /// assert_eq!(vec![3,2,1,4,5], v);
    /// ```
    ///
    /// # Return value
    ///
    /// A collection of a flattened [`Stack`]
    ///
    /// [`Stack`]: struct.Stack.html
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
    /// that yield true in the given filter function.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sabiwm::core::Stack;
    /// let stack = Stack::new(1, vec![2,3,4], vec![5,6,7]);
    /// let even_stack = stack.filter(|x| x % 2 == 0).unwrap();
    ///
    /// assert_eq!(vec![4,2,6], even_stack.integrate::<Vec<_>>());
    /// ```
    ///
    /// # Arguments
    ///
    /// `f` - the filter function to use for filtering
    ///
    /// # Return value
    ///
    /// Returns `Some(Stack)` if there are still items left after filtering
    /// or `None` if there is no element left after filtering.
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
            let down: Vec<T> = (vec![self.focus])
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
            let xs: Vec<T> = self.up.iter().skip(1).cloned().collect();
            let rs: Vec<T> = (vec![x]).into_iter().chain(self.down.clone().into_iter()).collect();
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

    /// Checks if the [`Stack`] is empty
    ///
    /// # Examples
    ///
    /// ```
    /// # use sabiwm::core::Stack;
    /// let stack = Stack::from(42);
    /// assert_eq!(false, stack.is_empty());
    /// ```
    ///
    /// # Return value
    ///
    /// `true` if the [`Stack`] is empty, `false` otherwise
    ///
    /// [`Stack`]: struct.Stack.html
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Checks if the given window is tracked by the stack
    ///
    /// ```
    /// # use sabiwm::core::Stack;
    /// let stack = Stack::from(42);
    /// assert_eq!(true, stack.contains(42));
    /// assert_eq!(false, stack.contains(23));
    /// ```
    ///
    /// # Arguments
    ///
    /// `t` - the element to search for
    ///
    /// # Return value
    ///
    /// `true` if the [`Stack`] contains the given element
    ///
    /// [`Stack`]: struct.Stack.html
    pub fn contains(&self, t: T) -> bool {
        trace!("checking if stack contains {:?}", t);
        self.integrate::<Vec<_>>().contains(&t)
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
