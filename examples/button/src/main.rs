use pixel_ui::prelude::*;
use winit::{
    dpi::PhysicalSize,
    error::EventLoopError,
    event::{ElementState, MouseButton},
    window::WindowAttributes,
};

const WIDTH: usize = 640;
const HEIGHT: usize = 480;

fn main() -> Result<(), EventLoopError> {
    let window_attributes = WindowAttributes::default()
        .with_inner_size(PhysicalSize::new(WIDTH as u32, HEIGHT as u32))
        .with_title("pixel-ui Button Test")
        ;

    let mut app = Application::builder()
        .expect("should be able to create application")
        .with_attributes(window_attributes);

    let (hidden, set_hidden) = app.create_signal(false);

    let mut button = Element::button(60, 25, Color::BLACK)
        .with_offset(300, 180);
    
    let mut rect = Element::rect(20, 20, Color::WHITE)
        .with_offset(100, 190);

    button.on_click(move |app, _el, ev| {
        if ev.button == MouseButton::Left && ev.state == ElementState::Released {
            set_hidden.update(app, |current| *current = !*current);
        }
    });

    button.on_resize(|app, el, ev| {
        let button = app.get_mut(el);

        button.set_offset((ev.size.x / 2) as isize, (ev.size.y / 2) as isize);
    });

    rect.on_rehydrate(move |app, el| {
        let hid = hidden.get(app);
        let element = app.get_mut(el);

        element.set_hidden(hid);
    });

    app.insert_element(button);
    app.insert_element(rect);

    app.run()
}
