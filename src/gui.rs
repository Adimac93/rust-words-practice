use druid::widget::{Button, Container, Controller, Flex, Label, Split, TextBox};
use druid::{AppLauncher, Color, Data, Env, Event, EventCtx, Lens, Widget, WidgetExt, WindowDesc};

#[derive(Clone, Data, Lens)]
struct AppState {
    name: String,
}

impl AppState {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}
fn build_ui() -> impl Widget<AppState> {
    Container::new(
        Flex::column()
            .with_flex_child(
                Button::new("Check").on_click(|ctx, a, env| {
                    println!("Hello!");
                }),
                1.0,
            )
            .with_flex_child(
                TextBox::new()
                    .with_placeholder("Guess")
                    .event(EventCtx::, &Event::KeyDown(a))
                    .lens(AppState::name),
                1.0,
            ),
    )
}

pub fn start_gui_mode() -> anyhow::Result<()> {
    let main_window = WindowDesc::new(build_ui())
        .window_size((600.0, 400.0))
        .title("My first Druid App");
    let initial_data = AppState { name: "".into() };
    AppLauncher::with_window(main_window)
        .launch(initial_data)
        .expect("Failed to launch application");

    Ok(())
}

struct TextBoxController;

impl Controller<String, TextBox<String>> for TextBoxController {
    fn event(
        &mut self,
        child: &mut TextBox<String>,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut String,
        env: &Env,
    ) {
        match event {
            Event::KeyDown(key) => {

                child.event(ctx, event, data, env),
            }
            _ => child.event(ctx, event, data, env),
        }
    }
}