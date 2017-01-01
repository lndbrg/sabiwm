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
        let map_request: &xcb::MapRequestEvent = xcb::cast_event(event);
        debug!("xcb map request for new window {:?}", map_request.window());
        xcb::map_window(&self.connection, map_request.window());
        self.connection.flush();
        Event::WindowCreated(map_request.window())
    }

    fn destroy_window(&self, event: &xcb::GenericEvent) -> Event<xcb::Window> {
        let destroy_notify: &xcb::DestroyNotifyEvent = xcb::cast_event(event);
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
            let cookie = xcb::change_window_attributes(connection, root, &values);
            let _ = cookie.request_check();
        }
    }

    fn acquire_root_window(connection: &xcb::Connection, screen_number: i32) -> xcb::Window {
        let setup = connection.get_setup();
        let screen = setup.roots().nth(screen_number as usize).unwrap();
        info!("screen is {}x{}",
              screen.width_in_pixels(),
              screen.height_in_pixels());
        screen.root()
    }
}

impl Backend for Xcb {
    type Window = xcb::Window;

    fn new() -> Result<Xcb> {
        info!("connecting to default display");
        let (conn, screen_number) =
            xcb::Connection::connect(None).map_err(|_| "unable to connect to display")?;

        let root = Xcb::acquire_root_window(&conn, screen_number);
        debug!("acquired root window {:?}", root);
        Xcb::set_event_mask(&conn, root);
        conn.flush();

        Ok(Xcb {
            connection: conn,
            root: root,
        })
    }

    fn is_dock(&self, window: Self::Window) -> bool {
        trace!("checking if {:?} is a dock", window);
        unimplemented!();
    }

    fn is_window(&self, window: Self::Window) -> bool {
        match xcb::get_window_attributes(&self.connection, window).get_reply() {
            Ok(reply) => !reply.override_redirect(),
            _ => true,
        }
    }

    fn screens(&self) -> Vec<Rectangle> {
        trace!("getting screen layout information");
        unimplemented!();
    }

    fn number_of_screens(&self) -> usize {
        self.connection.get_setup().roots().fold(0, |acc, _| acc + 1)
    }

    fn window_name(&self, window: Self::Window) -> Option<String> {
        trace!("retrieving name of window {:?}", window);
        let atom_net_wm_name = xcb::intern_atom(&self.connection, false, "_NET_WM_NAME")
            .get_reply()
            .expect("failed to intern _NET_WM_NAME atom")
            .atom();

        xcb::get_property(&self.connection,
                          false,
                          window,
                          atom_net_wm_name,
                          xcb::ATOM_STRING,
                          0,
                          u32::max_value())
            .get_reply()
            .ok()
            .and_then(|reply| if reply.value_len() > 0 {
                Some(String::from_utf8(reply.value().to_vec()).unwrap())
            } else {
                None
            }
            ).or_else(||
              xcb::get_property(&self.connection,
                                false,
                                window,
                                xcb::ATOM_WM_NAME,
                                xcb::ATOM_STRING,
                                0,
                                u32::max_value())
                .get_reply()
                .ok()
                .and_then(|reply| Some(String::from_utf8(reply.value().to_vec()).unwrap()))
        )
    }

    fn class_name(&self, window: Self::Window) -> String {
        trace!("retrieving class name of window {:?}", window);
        unimplemented!();
    }

    fn windows(&self) -> Result<Vec<Self::Window>> {
        Ok(xcb::query_tree(&self.connection, self.root)
            .get_reply()
            .map_err(|_| "unable to query xcb tree")?
            .children()
            .iter()
            .cloned()
            .collect())
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
        xcb::map_window(&self.connection, window);
    }

    fn hide_window(&self, window: Self::Window) {
        xcb::unmap_window(&self.connection, window);
    }

    fn focus_window(&self, window: Self::Window) {
        xcb::set_input_focus(&self.connection, 0, window, xcb::CURRENT_TIME);
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
                        Event::Unknown
                    }
                }
            }
            _ => Event::Unknown,
        }
    }
}
