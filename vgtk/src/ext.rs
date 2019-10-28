use gdk_pixbuf::Pixbuf;
use gio::ApplicationFlags;
use glib::{GString, Object};
use gtk::{
    Application, Box, BoxExt, GtkWindowExt, Image, ImageExt, Label, LabelExt, Window,
    WindowPosition, WindowType,
};

pub trait ApplicationHelpers {
    fn new_unwrap(application_id: Option<&str>, flags: ApplicationFlags) -> Application;
}

impl ApplicationHelpers for Application {
    fn new_unwrap(application_id: Option<&str>, flags: ApplicationFlags) -> Application {
        Application::new(application_id, flags).expect("unable to create Application object")
    }
}

pub trait WindowExtHelpers: GtkWindowExt {
    fn get_default_height(&self) -> i32;
    fn set_default_height(&self, default_height: i32);
    fn get_default_width(&self) -> i32;
    fn set_default_width(&self, default_width: i32);
    fn get_has_toplevel_focus(&self) -> bool;
    fn get_is_active(&self) -> bool;
    fn get_is_maximized(&self) -> bool;
    fn get_type(&self) -> WindowType;
    fn get_window_position(&self) -> WindowPosition;
    fn set_window_position(&self, window_position: WindowPosition);
}

impl WindowExtHelpers for Window {
    fn get_default_height(&self) -> i32 {
        self.get_property_default_height()
    }

    fn set_default_height(&self, default_height: i32) {
        self.set_property_default_height(default_height)
    }

    fn get_default_width(&self) -> i32 {
        self.get_property_default_width()
    }

    fn set_default_width(&self, default_width: i32) {
        self.set_property_default_width(default_width)
    }

    fn get_has_toplevel_focus(&self) -> bool {
        self.get_property_has_toplevel_focus()
    }

    fn get_is_active(&self) -> bool {
        self.get_property_is_active()
    }

    fn get_is_maximized(&self) -> bool {
        self.get_property_is_maximized()
    }

    fn get_type(&self) -> WindowType {
        self.get_property_type()
    }

    fn get_window_position(&self) -> WindowPosition {
        self.get_property_window_position()
    }

    fn set_window_position(&self, window_position: WindowPosition) {
        self.set_property_window_position(window_position)
    }
}

pub trait BoxExtHelpers: BoxExt {
    fn get_child_center_widget(&self, child: &Object) -> bool;
    fn set_child_center_widget(&self, child: &Object, center: bool);
}

impl BoxExtHelpers for Box {
    fn get_child_center_widget(&self, _child: &Object) -> bool {
        // Always compare true, it's all taken care of in add_child().
        true
    }

    fn set_child_center_widget(&self, _child: &Object, _center: bool) {
        // This is handled by add_child() rules. The setter is a no-op.
    }
}

pub trait ImageExtHelpers: ImageExt {
    fn set_pixbuf(&self, pixbuf: Option<Pixbuf>);
}

impl ImageExtHelpers for Image {
    fn set_pixbuf(&self, pixbuf: Option<Pixbuf>) {
        self.set_from_pixbuf(pixbuf.as_ref());
    }
}

pub trait LabelExtHelpers: LabelExt {
    fn get_markup(&self) -> Option<GString>;
}

impl LabelExtHelpers for Label {
    fn get_markup(&self) -> Option<GString> {
        self.get_label()
    }
}
