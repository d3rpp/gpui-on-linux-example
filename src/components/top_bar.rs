use super::Button;

use gpui::*;

#[derive(IntoElement)]
pub struct TopBar {
	title: SharedString,
	height: Length
}

impl TopBar {
	pub fn new(title: impl Into<SharedString>, height: impl Into<Length>) -> Self {
		TopBar {
			title: title.into(),
			height: height.into()
		}
	}
}

impl RenderOnce for TopBar {
	fn render(self, cx: &mut WindowContext) -> impl IntoElement {
		div()
			.bg(rgb(0x181818))
			.w_full()
			.h(self.height)
			.flex()
			.flex_row()
			.justify_start()
			.items_center()
			.pl_1()
			.child(
				div()
					.h_full()
					.w_full()
					.cursor_move()
					.on_mouse_move(|ev, cx| {
						if ev.dragging() {
							cx.start_system_move()
						}
					})
					.child(self.title)
					.child(spacer())
			)
			.child(
				Button {
					hover_colour: rgba(0xffffff20).into(),
					click_action: Box::new(|_ev, cx| {cx.minimize_window()}),

					icon: "_",
					width: self.height,
					height: self.height
				}
			)
			.child(
				Button {
					hover_colour: rgba(0xffffff20).into(),
					click_action: Box::new(|_ev, cx| {cx.zoom_window()}),

					icon: if cx.is_maximized() { "-" } else { "^" },
					width: self.height,
					height: self.height
				}
			)
			.child(
				Button {
					hover_colour: rgb(0xff0000).into(),
					click_action: Box::new(|_ev, cx| {cx.quit()}),

					icon: "x",
					width: self.height,
					height: self.height
				}
			)
	}
}

fn spacer() -> impl IntoElement {
	div()
		.w_full()
		.h_full()
}