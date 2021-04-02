extern crate conrod;
extern crate find_folder;
use conrod::backend::glium::glium::{self, Surface};
use conrod::{widget, Positionable, Colorable, Widget, widget_ids};

const WIDTH: u32 = 400;
const HEIGHT: u32 = 400;

fn main() {
    println!("Hello, world!");

    let mut events_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new()
                    .with_title("Hello Conrod")
                    .with_dimensions(WIDTH, HEIGHT);
    let context = glium::glutin::ContextBuilder::new()
                    .with_vsync(true)
                    .with_multisampling(4);
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let mut ui = conrod::UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();
    let image_map = conrod::image::Map::<glium::texture::Texture2d>::new();
    let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();

    widget_ids!(struct Ids { text });
    let ids = Ids::new(ui.widget_id_generator());

    let assets = find_folder::Search::KidsThenParents(3, 5)
        .for_folder("assets")
        .unwrap();
    let font_path = assets.join("NotoSans-Regular.ttf");
    ui.fonts.insert_from_file(font_path).unwrap();

    let mut event_loop = EventLoop::new();
    'main: loop {
        for event in event_loop.next(&mut events_loop) {
            if let Some(event) = conrod::backend::winit::convert_event(
                event.clone(),
                &display
            ) {
                ui.handle_event(event);
            }

            match event {
                // handle window events
                glium::glutin::Event::WindowEvent { event, .. } => match event {
                    glium::glutin::WindowEvent::Closed => break 'main,
                    _ => (),
                }
                _ => (),
            }
        }

        if let Some(primitives) = ui.draw_if_changed() {
            renderer.fill(&display, primitives, &image_map);
            let mut target = display.draw();
            target.clear_color(0.0, 1.0, 0.0, 1.0);

            let ui_cell = &mut ui.set_widgets(); 
            widget::Text::new("Hello world!")
                .middle_of(ui_cell.window)
                .color(conrod::color::BLACK)
                .font_size(32)
                .set(ids.text, ui_cell);

            renderer.draw(&display, &mut target, &image_map).unwrap();
            target.finish().unwrap();
        }
    }

    println!("exited main loop");
}

pub struct EventLoop {
    ui_needs_update: bool,
    last_update: std::time::Instant,
}

impl EventLoop {
    pub fn new() -> Self {
        EventLoop {
            last_update: std::time::Instant::now(),
            ui_needs_update: true,
        }
    }

    // produce an iterator yielding all available events
    pub fn next(&mut self, events_loop: &mut glium::glutin::EventsLoop) -> 
    Vec<glium::glutin::Event> {
        // caps at 60 FPS
        let last_update = self.last_update;
        let sixteen_ms = std::time::Duration::from_millis(16);
        let duration_since_last_update = std::time::Instant::now().duration_since(last_update);
        if duration_since_last_update < sixteen_ms {
            std::thread::sleep(sixteen_ms - duration_since_last_update);
        }

        // collect all pending events
        let mut events = Vec::new();
        events_loop.poll_events(|event| events.push(event));

        // if there are no events and the UI doesn't need updating, wait
        // for the next event
        if events.is_empty() && !self.ui_needs_update {
            events_loop.run_forever(|event| {
                events.push(event);
                glium::glutin::ControlFlow::Break
            });
        }

        self.ui_needs_update = false;
        self.last_update = std::time::Instant::now();
        
        events
    }

    /// Notifies the event loop that `Ui` needs another update whether or not
    /// there are any pending events
    /// 
    /// mostly used if part of UI is still animating and needs more updates
    pub fn needs_update(&mut self) {
        self.ui_needs_update = true;
    }
}
