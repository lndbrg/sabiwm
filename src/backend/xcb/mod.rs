use backend::{Backend, Event};
use core::Rectangle;
use errors::*;
use xcb;

pub struct Xcb {
    connection: xcb::Connection,
    root: xcb::ffi::xproto::xcb_window_t,
}

impl Xcb {
    fn create_window(&self, event: &xcb::GenericEvent) -> Event<xcb::Window> {
        let map_request: &xcb::MapRequestEvent = xcb::cast_event(&event);
        debug!("xcb map request for new window {:?}", map_request.window());
        xcb::map_window(&self.connection, map_request.window());
        self.connection.flush();
        Event::WindowCreated(map_request.window())
    }

    fn destroy_window(&self, event: &xcb::GenericEvent) -> Event<xcb::Window> {
        let destroy_notify: &xcb::DestroyNotifyEvent = xcb::cast_event(&event);
        debug!("xcb destroy notification for window {:?}",
               destroy_notify.window());
        self.connection.flush();
        Event::WindowClosed(destroy_notify.window())
    }

    fn set_event_mask(connection: &xcb::Connection, root: xcb::Window) {
        debug!("setting root window properties");
        let values =
            [(xcb::CW_EVENT_MASK,
              xcb::EVENT_MASK_SUBSTRUCTURE_NOTIFY | xcb::EVENT_MASK_SUBSTRUCTURE_REDIRECT |
              xcb::EVENT_MASK_STRUCTURE_NOTIFY | xcb::EVENT_MASK_ENTER_WINDOW |
              xcb::EVENT_MASK_LEAVE_WINDOW | xcb::EVENT_MASK_PROPERTY_CHANGE |
              xcb::EVENT_MASK_BUTTON_PRESS |
              xcb::EVENT_MASK_BUTTON_RELEASE | xcb::EVENT_MASK_FOCUS_CHANGE)];

        {
            let cookie = xcb::change_window_attributes(&connection, root, &values);
            let _ = cookie.request_check();
        }
    }
}

impl Backend for Xcb {
    type Window = xcb::Window;

    fn new() -> Result<Xcb> {
        info!("connecting to default display");
        let (conn, screen_number) =
            xcb::Connection::connect(None).map_err(|_| "unable to connect to display")?;

        let root = {
            let setup = conn.get_setup();
            let screen = setup.roots().nth(screen_number as usize).unwrap();
            info!("screen is {}x{}",
                  screen.width_in_pixels(),
                  screen.height_in_pixels());
            screen.root()
        };
        debug!("acquired root window {:?}", root);
        Xcb::set_event_mask(&conn, root);
        conn.flush();

        Ok(Xcb {
            connection: conn,
            root: root,
        })
    }

    fn is_dock(&self, window: Self::Window) -> bool {
        unimplemented!();
    }

    fn is_window(&self, window: Self::Window) -> bool {
        match xcb::get_window_attributes(&self.connection, window).get_reply() {
            Ok(reply) => !reply.override_redirect(),
            _ => true,
        }
    }
    fn screens(&self) -> Vec<Rectangle> {
        unimplemented!();
    }
    fn number_of_screens(&self) -> usize {
        self.connection.get_setup().roots().fold(0, |acc, _| acc + 1)
    }
    fn window_name(&self, window: Self::Window) -> String {
        unimplemented!();
    }
    fn class_name(&self, window: Self::Window) -> String {
        unimplemented!();
    }
    fn windows(&self) -> Vec<Self::Window> {
        unimplemented!();
    }
    fn resize_window(&self, window: Self::Window, width: u32, height: u32) {
        trace!("resizing window {:?} to {}x{}", window, width, height);
        let values = [(xcb::CONFIG_WINDOW_WIDTH as u16, width),
                      (xcb::CONFIG_WINDOW_HEIGHT as u16, height)];
        xcb::configure_window(&self.connection, window, &values);
        self.connection.flush();
    }
    fn move_window(&self, window: Self::Window, x: u32, y: u32) {
        trace!("moving window {:?} to {}x{}", window, x, y);
        let values = [(xcb::CONFIG_WINDOW_X as u16, x), (xcb::CONFIG_WINDOW_Y as u16, y)];
        xcb::configure_window(&self.connection, window, &values);
        self.connection.flush();
    }
    fn show_window(&self, window: Self::Window) {
        unimplemented!();
    }
    fn hide_window(&self, window: Self::Window) {
        unimplemented!();
    }
    fn focus_window(&self, window: Self::Window) {
        unimplemented!();
    }
    fn event(&self) -> Event<Self::Window> {
        trace!("waiting for next event");
        let event = self.connection.wait_for_event();

        match event {
            Some(event) => {
                debug!("received event");
                let response_type = event.response_type();
                match response_type {
                    xcb::MAP_REQUEST => self.create_window(&event),
                    xcb::DESTROY_NOTIFY => self.destroy_window(&event),
                    _ => {
                        warn!("unknown request {:?}", response_type);
                        Event::UnknownEvent
                    }
                }
            }
            _ => Event::UnknownEvent,
        }
    }
}
