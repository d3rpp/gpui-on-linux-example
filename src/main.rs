#![feature(fn_traits)]

mod components;

use components::TopBar;
use gpui::*;

struct AppInfo {
    ver: String,
}

impl Global for AppInfo {}

struct WindowState {
    count: u32,
    ev: Option<String>,
}

struct WindowContent {
    text: SharedString,
    window_state: Model<WindowState>,
}

impl Render for WindowContent {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let ver = cx.global::<AppInfo>();
        let window_state_ref = self.window_state.clone();

        let mut count = 0;
        let mut ev_str = "".to_string();

        cx.read_model(&window_state_ref, |state, _cx| {
            count = state.count;
            ev_str = state.ev.clone().unwrap_or_default();
        });

        let update_button = div()
            .on_any_mouse_down(move |ev, cx| {
                cx.update_model(&window_state_ref, move |state, _cx| {
                    state.count += 1;
                    state.ev = Some(format!("{ev:?}"));
                });
                cx.refresh()
            })
            .w_auto()
            .max_w_1_2()
            .h_auto()
            .text_color(rgb(0xffffff))
            .bg(rgb(0x555555))
            .child(format!("Ver: {} Count: {count} - ev: {ev_str}", &ver.ver));

        div()
            .flex()
            .flex_col()
            .bg(rgb(0x111111))
            .text_color(rgb(0xdddddd))
            .size_full()
            .text_xl()
            .child(TopBar::new("GPUI Test", px(32f32)))
            .child(self.text.clone())
            .child(update_button)
    }
}

fn main() {
    App::new().run(|cx: &mut AppContext| {
        cx.set_global(AppInfo {
            ver: "Gamer".into(),
        });

        let options = WindowOptions {
            bounds: None,
            display_id: None,

            titlebar: Some(TitlebarOptions {
                title: Some("Zed".into()),
                ..Default::default()
            }),

            window_background: WindowBackgroundAppearance::Opaque,
            focus: true,
            show: true,
            kind: WindowKind::Normal,
            is_movable: true,
            fullscreen: false,
            app_id: Some("zed-linux-test".into()),
        };

        cx.open_window(options, |cx| {
            let window_state = cx.new_model(|_cx| WindowState { count: 0, ev: None });

            cx.new_view(|_| WindowContent {
                text: "Hello, World!".into(),
                window_state,
            })
        });
    });
}
