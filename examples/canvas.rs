//!
//! A simple demonstration of how to construct and use Canvasses by splitting up the window.
//!

#[macro_use] extern crate conrod;
extern crate find_folder;
extern crate piston_window;


use conrod::{Canvas, Theme, Widget, color};
use piston_window::{EventLoop, G2dTexture, OpenGL, PistonWindow, UpdateEvent, WindowSettings};



fn main() {
    const WIDTH: u32 = 800;
    const HEIGHT: u32 = 600;

    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;
    
    // Construct the window.
    let mut window: PistonWindow =
        WindowSettings::new("Canvas Demo", [WIDTH, HEIGHT])
            .opengl(opengl).exit_on_esc(true).vsync(true).build().unwrap();

    // construct our `Ui`.
    let mut ui = conrod::Ui::new(Theme::default());

    // Add a `Font` to the `Ui`'s `font::Map` from file.
    {
        let assets = find_folder::Search::KidsThenParents(3, 5)
            .for_folder("assets").unwrap();
        let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");
        ui.fonts.insert_from_file(font_path).unwrap();
    }

    // Create a texture cache in which we can cache text on the GPU.
    let mut text_texture_cache: G2dTexture<'static> = {
        const BUFFER_LEN: usize = WIDTH as usize * HEIGHT as usize;
        const INIT: [u8; BUFFER_LEN] = [128; BUFFER_LEN];
        let factory = &mut window.factory;
        let settings = piston_window::TextureSettings::new();
        G2dTexture::from_memory_alpha(factory, &INIT, WIDTH, HEIGHT, &settings)
            .expect("Failed to create G2dTexture::from_memory_alpha")
    };

    window.set_ups(60);

    // Poll events from the window.
    while let Some(event) = window.next() {
        ui.handle_event(event.clone());

        event.update(|_| {
            ui.set_widgets(set_widgets)
        });

        window.draw_2d(&event, |c, g| {
            if let Some(primitives) = ui.draw_if_changed() {

                // Data and functions for rendering the primitives.
                let renderer = conrod::backend::draw_piston::Renderer {
                    context: c,
                    graphics: g,
                    texture_cache: &mut text_texture_cache,
                    // A type used for passing the `texture_cache` used for caching and rendering
                    // `Text` to the function for rendering.
                    cache_queued_glyphs: |graphics: &mut piston_window::G2d,
                                          cache: &mut G2dTexture<'static>,
                                          rect: conrod::text::RtRect<u32>,
                                          data: &[u8]| {
                        use piston_window::texture::UpdateTexture;
                        let dim = [rect.width(), rect.height()];
                        let format = piston_window::texture::Format::Rgba8;
                        let encoder = &mut graphics.encoder;
                        UpdateTexture::update(cache, encoder, format, data, dim)
                            .expect("Failed to update texture");
                    },
                    // A function that returns some texture `T` for the given `texture::Id`. We
                    // have no `Image` widgets, so no need to implement this.
                    get_texture: |_id| None,
                };

                conrod::backend::draw_piston::primitives(primitives, renderer);
            }
        });
    }

}


// Draw the Ui.
fn set_widgets(ref mut ui: conrod::UiCell) {
    use conrod::{Button, Colorable, Labelable, Positionable, Sizeable, Tabs, Text, WidgetMatrix};

    // Construct our main `Canvas` tree.
    Canvas::new().flow_down(&[
        (HEADER, Canvas::new().color(color::BLUE).pad_bottom(20.0)),
        (BODY, Canvas::new().length(300.0).flow_right(&[
            (LEFT_COLUMN, Canvas::new().color(color::LIGHT_ORANGE).pad(20.0)),
            (MIDDLE_COLUMN, Canvas::new().color(color::ORANGE)),
            (RIGHT_COLUMN, Canvas::new().color(color::DARK_ORANGE).pad(20.0)),
        ])),
        (FOOTER, Canvas::new().color(color::BLUE).scroll_kids_vertically()),
    ]).set(MASTER, ui);

    // A scrollbar for the `FOOTER` canvas.
    conrod::Scrollbar::y_axis(FOOTER).auto_hide(true).set(FOOTER_SCROLLBAR, ui);

    // Now we'll make a couple floating `Canvas`ses.
    let floating = Canvas::new().floating(true).w_h(110.0, 150.0).label_color(color::WHITE);
    floating.middle_of(LEFT_COLUMN).title_bar("Blue").color(color::BLUE).set(FLOATING_A, ui);
    floating.middle_of(RIGHT_COLUMN).title_bar("Orange").color(color::LIGHT_ORANGE).set(FLOATING_B, ui);

    // Here we make some canvas `Tabs` in the middle column.
    Tabs::new(&[(TAB_FOO, "FOO"), (TAB_BAR, "BAR"), (TAB_BAZ, "BAZ")])
        .wh_of(MIDDLE_COLUMN)
        .color(color::BLUE)
        .label_color(color::WHITE)
        .middle_of(MIDDLE_COLUMN)
        .set(TABS, ui);

    Text::new("Fancy Title").color(color::LIGHT_ORANGE).font_size(48).middle_of(HEADER).set(TITLE, ui);
    Text::new("Subtitle").color(color::BLUE.complement()).mid_bottom_of(HEADER).set(SUBTITLE, ui);

    Text::new("Top Left")
        .color(color::LIGHT_ORANGE.complement())
        .top_left_of(LEFT_COLUMN)
        .set(TOP_LEFT, ui);

    Text::new("Bottom Right")
        .color(color::DARK_ORANGE.complement())
        .bottom_right_of(RIGHT_COLUMN)
        .set(BOTTOM_RIGHT, ui);

    Text::new("Foo!").color(color::WHITE).font_size(36).middle_of(TAB_FOO).set(FOO_LABEL, ui);
    Text::new("Bar!").color(color::WHITE).font_size(36).middle_of(TAB_BAR).set(BAR_LABEL, ui);
    Text::new("BAZ!").color(color::WHITE).font_size(36).middle_of(TAB_BAZ).set(BAZ_LABEL, ui);

    let footer_wh = ui.wh_of(FOOTER).unwrap();
    WidgetMatrix::new(COLS, ROWS)
        .w_h(footer_wh[0], footer_wh[1] * 2.0)
        .mid_top_of(FOOTER)
        .each_widget(|n, _col, _row| {
            Button::new()
                .color(color::BLUE.with_luminance(n as f32 / (COLS * ROWS) as f32))
                .react(move || println!("Hey! {:?}", n))
        })
        .set(BUTTON_MATRIX, ui);

    Button::new().color(color::RED).w_h(30.0, 30.0).middle_of(FLOATING_A)
        .react(|| println!("Bing!"))
        .set(BING, ui);
    Button::new().color(color::RED).w_h(30.0, 30.0).middle_of(FLOATING_B)
        .react(|| println!("Bong!"))
        .set(BONG, ui);
}


// Button matrix dimensions.
const ROWS: usize = 10;
const COLS: usize = 24;


// Generate a unique `WidgetId` for each widget.
widget_ids! {

    // Canvas IDs.
    MASTER,
    HEADER,
    BODY,
    LEFT_COLUMN,
    MIDDLE_COLUMN,
    RIGHT_COLUMN,
    FOOTER,
    FOOTER_SCROLLBAR,
    FLOATING_A,
    FLOATING_B,
    TABS,
    TAB_FOO,
    TAB_BAR,
    TAB_BAZ,

    // Widget IDs.
    TITLE,
    SUBTITLE,
    TOP_LEFT,
    BOTTOM_RIGHT,
    FOO_LABEL,
    BAR_LABEL,
    BAZ_LABEL,
    BUTTON_MATRIX,
    BING,
    BONG,

}
