use pixel_ui::prelude::*;
use winit::{dpi::PhysicalSize, error::EventLoopError, event::{ElementState, MouseButton}, window::WindowAttributes};

const WIDTH: usize = 640;
const HEIGHT: usize = 480;

fn main() -> Result<(), EventLoopError> {
    let window_attributes = WindowAttributes::default()
        .with_inner_size(PhysicalSize::new(WIDTH as u32, HEIGHT as u32))
        .with_title("pixel-ui Button Test");


    let mut app = Application::builder()
        .expect("should be able to create application")
        .with_attributes(window_attributes);

    let button = Button::new(WIDTH, HEIGHT);
    let mut element = Element::new(button);
    element.on_click(|app, _el, ev| {
        if ev.button == MouseButton::Left && ev.state == ElementState::Released {
            app.set_background(!app.get_background());
        }
    });

    app.insert_element(element);

    app.run()
}
