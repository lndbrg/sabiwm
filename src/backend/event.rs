use core::Rectangle;

pub enum Event<Window> {
    BackendChanged,
    WindowCreated(Window),
    WindowClosed(Window),
    WindowRevealed(Window),
    WindowChangeRequest(Window, Rectangle),
    MouseEnter(Window),
    MouseLeave(Window),
    ButtonPressed(Window, Window),
    ButtonReleased,
    KeyPressed(Window),
    UnknownEvent,
}
