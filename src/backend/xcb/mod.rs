use backend::{Backend, Event};
use core::Rectangle;
use errors::*;
use xcb;

/// The Xcb backend. This backend shall be the default,
/// until Wayland becomes the default environment.
pub struct Xcb {
    connection: xcb::Connection,
    root: xcb::Window,
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

    fn get_interned_atom(&self, atom: &str) -> Result<xcb::Atom> {
        Ok(xcb::intern_atom(&self.connection, false, atom)
            .get_reply()
            .map_err(|_| format!("unable to get atom {}", atom))?
            .atom())
    }

    fn get_string_atom(&self, atom: xcb::Atom, window: xcb::Window) -> Result<String> {
        let reply = xcb::get_property(&self.connection,
                                      false,
                                      window,
                                      atom,
                                      xcb::ATOM_STRING,
                                      0,
                                      u32::max_value()).get_reply()
            .map_err(|err| format!("{:?}", err))?;
        match String::from_utf8(reply.value().to_vec()) {
            Ok(ref name) if name.len() > 0 => Ok(name.clone()),
            _ => bail!("unable to get property"),
        }
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
        let dock = try_or_false!(self.get_interned_atom("_NET_WM_WINDOW_TYPE_DOCK"));
        let desk = try_or_false!(self.get_interned_atom("_NET_WM_WINDOW_TYPE_DESKTOP"));
        let window_type = try_or_false!(self.get_interned_atom("_NET_WM_WINDOW_TYPE"));

        xcb::get_property(&self.connection,
                          false,
                          window,
                          window_type,
                          xcb::ATOM_ATOM,
                          0,
                          u32::max_value())
            .get_reply()
            .iter()
            .any(|x| x.type_() == dock || x.type_() == desk)
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
        self.connection.get_setup().roots_len() as usize
    }

    fn window_name(&self, window: Self::Window) -> Result<String> {
        trace!("retrieving name of window {:?}", window);
        // First, try to get the EWMH atom and the window name.
        // If that fails, fall back to WM_NAME.
        let atom = self.get_interned_atom("_NET_WM_NAME")?;
        self.get_string_atom(atom, window)
            .or_else(|_| self.get_string_atom(xcb::ATOM_WM_NAME, window))
    }

    fn class_name(&self, window: Self::Window) -> Result<String> {
        trace!("retrieving class name of window {:?}", window);
        self.get_string_atom(xcb::ATOM_WM_CLASS, window)
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
