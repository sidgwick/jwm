use crate::xlib::{
    x_create_simple_window, x_default_colormap, x_default_depth, x_default_gc, x_default_screen,
    x_default_visual, x_display_height, x_display_width, x_grab_server, x_intern_atom,
    x_max_cmaps_of_screen, x_open_display, x_root_window, x_screen_of_display,
    x_set_graphics_exposures,
};
use log::info;
use std::fmt;
use std::fmt::Formatter;
use x11::xlib::{Display, Visual, _XGC};

pub struct Root {
    display: *mut Display,
    visual: *mut Visual,
    gc: *mut _XGC,
    window: u64,
    screen: i32,
    width: i32,
    height: i32,
    depth: i32,
    colormap: u64,
    colormap_count: i32,
}

impl fmt::Debug for Root {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Root")
            .field("display", &format!("{:p}", self.display))
            .field("visual", &format!("{:p}", self.visual))
            .field("gc", &format!("{:p}", self.gc))
            .field("window", &self.window)
            .field("screen", &self.screen)
            .field("width", &self.width)
            .field("height", &self.height)
            .field("depth", &self.depth)
            .field("colormap", &self.colormap)
            .field("colormap_count", &self.colormap_count)
            .finish()
    }
}

/// Open a connection to the X server
fn open_connection() -> Root {
    let display = match x_open_display() {
        Some(display) => display,
        None => panic!("error: could not open display"),
    };

    let screen = x_default_screen(display);
    let window = x_root_window(display, screen);
    let width = x_display_width(display, screen);
    let height = x_display_height(display, screen);
    let depth = x_default_depth(display, screen);
    let visual = match x_default_visual(display, screen) {
        Some(visual) => visual,
        None => panic!("error: could not get default visual"),
    };

    let colormap = x_default_colormap(display, screen);

    let gc = match x_default_gc(display, screen) {
        Some(gc) => gc,
        None => panic!("error: unable to get root GC"),
    };

    let colormap_count = match x_screen_of_display(display, screen) {
        Some(screen) => x_max_cmaps_of_screen(screen),
        None => panic!("error: unable to found screen of display"),
    };

    x_set_graphics_exposures(display, gc, false);

    Root {
        display: display,
        screen: screen,
        window: window,
        width: width,
        height: height,
        depth: depth,
        visual: visual,
        colormap: colormap,
        gc: gc,
        colormap_count: colormap_count,
    }
}

pub fn setup_connection() {
    let mut root = open_connection();

    let supporting_window = x_create_simple_window(root.display, root.window, 0, 0, 1, 1, 0, 0, 0);

    let name = format!("WM_S{}", root.screen);
    let manager_selection = x_intern_atom(root.display, &name, false);

    x_grab_server(root.display);

    info!("root structure is: {:?}", root);

    //    win = JXGetSelectionOwner(display, managerSelection);
    //    if(win != None) {
    //       JXSelectInput(display, win, StructureNotifyMask);
    //    }
    //    JXSetSelectionOwner(display, managerSelection,
    //                        supportingWindow, CurrentTime);
    //    UngrabServer();
    //
    //    /* Wait for the current selection owner to give up the selection. */
    //    if(win != None) {
    //       /* Note that we need to wait for the current selection owner
    //        * to exit before we can expect to select SubstructureRedirectMask. */
    //       XIfEvent(display, &event, SelectionReleased, (XPointer)&win);
    //       JXSync(display, False);
    //    }
    //
    //    event.xclient.display = display;
    //    event.xclient.type = ClientMessage;
    //    event.xclient.window = rootWindow;
    //    event.xclient.message_type = JXInternAtom(display, managerProperty, False);
    //    event.xclient.format = 32;
    //    event.xclient.data.l[0] = CurrentTime;
    //    event.xclient.data.l[1] = managerSelection;
    //    event.xclient.data.l[2] = supportingWindow;
    //    event.xclient.data.l[3] = 2;
    //    event.xclient.data.l[4] = 0;
    //    JXSendEvent(display, rootWindow, False, StructureNotifyMask, &event);
    //    JXSync(display, False);
    //
    //    JXSetErrorHandler(ErrorHandler);
    //
    //    clientContext = XUniqueContext();
    //    frameContext = XUniqueContext();
    //
    //    /* Set the events we want for the root window.
    //     * Note that asking for SubstructureRedirect will fail
    //     * if another window manager is already running.
    //     */
    //    attr.event_mask
    //       = SubstructureRedirectMask
    //       | SubstructureNotifyMask
    //       | StructureNotifyMask
    //       | PropertyChangeMask
    //       | ColormapChangeMask
    //       | ButtonPressMask
    //       | ButtonReleaseMask
    //       | PointerMotionMask | PointerMotionHintMask;
    //    JXChangeWindowAttributes(display, rootWindow, CWEventMask, &attr);
    //
    //    memset(&sa, 0, sizeof(sa));
    //    sa.sa_flags = 0;
    //    sa.sa_handler = HandleExit;
    //    sigaction(SIGTERM, &sa, NULL);
    //    sigaction(SIGINT, &sa, NULL);
    //    sigaction(SIGHUP, &sa, NULL);
    //
    //    sa.sa_handler = HandleChild;
    //    sigaction(SIGCHLD, &sa, NULL);
    //
    // #ifdef USE_SHAPE
    //    haveShape = JXShapeQueryExtension(display, &shapeEvent, &shapeError);
    //    if (haveShape) {
    //       Debug("shape extension enabled");
    //    } else {
    //       Debug("shape extension disabled");
    //    }
    // #endif
    //
    // #ifdef USE_XRENDER
    //    haveRender = JXRenderQueryExtension(display, &renderEvent, &renderError);
    //    if(haveRender) {
    //       Debug("render extension enabled");
    //    } else {
    //       Debug("render extension disabled");
    //    }
    // #endif
    //
    //    /* Make sure we have input focus. */
    //    win = None;
    //    JXGetInputFocus(display, &win, &revert);
    //    if(win == None) {
    //       JXSetInputFocus(display, rootWindow, RevertToParent, CurrentTime);
    //    }
    //
    //    initializing = 0;
}
