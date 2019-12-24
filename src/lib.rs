use seed::{prelude::*, *};
use web_sys::KeyboardEvent;

struct Model {
    pub cursor: i32,
    pub nums: Vec<i32>,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            cursor: 0,
            nums: (1..100).collect(),
        }
    }
}

#[derive(Clone)]
enum Msg {
    KeyPressed(KeyboardEvent),
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::KeyPressed(ev) => match ev.key().as_ref() {
            "j" => {
                model.cursor += 1;
                scroll_active_into_view();
            }
            "k" => {
                if model.cursor < 1 {
                    return;
                };

                model.cursor -= 1;

                if model.cursor <= 1 {
                    scroll_to_top();
                } else {
                    scroll_active_into_view();
                }
            }
            _ => (),
        },
    }
}

#[wasm_bindgen]
extern "C" {
    fn scroll_active_into_view();
    fn scroll_to_top();
}

fn view(model: &Model) -> impl View<Msg> {
    vec![
        h1![format!(
            "cursor: {}, nums: {}",
            model.cursor,
            model.nums.len()
        ),],
        div![
            attrs! {
                At::Class => "nums"
            },
            model
                .nums
                .iter()
                .map(|x| {
                    div![
                        attrs! {
                            At::Class => if *x == model.cursor { "active" } else { "" }
                        },
                        format!("{}", x)
                    ]
                })
                .collect::<Vec<_>>()
        ],
    ]
}

fn window_events(_: &Model) -> Vec<seed::events::Listener<Msg>> {
    vec![keyboard_ev("keydown", Msg::KeyPressed)]
}

#[wasm_bindgen(start)]
pub fn render() {
    App::builder(update, view)
        .window_events(window_events)
        .build_and_start();
    log("started");
}
