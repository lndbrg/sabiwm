use backend::Event;
use core::Rectangle;

pub trait Backend {
    type Window;

    fn is_dock(&self, window: Self::Window) -> bool;
    fn screens(&self) -> Vec<Rectangle>;
    fn number_of_screens(&self) -> usize;
    fn window_name(&self) -> String;
    fn class_name(&self) -> String;
    fn windows(&self) -> Vec<Self::Window>;
    fn resize_window(&self, window: Self::Window, width: u32, height: u32);
    fn move_window(&self, window: Self::Window, x: u32, y: u32);
    fn show_window(&self, window: Self::Window);
    fn hide_window(&self, window: Self::Window);
    fn focus_window(&self, window: Self::Window);
    fn event(&self) -> Event<Self::Window>;
}
