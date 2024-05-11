use gpui::*;

type ButtonClickAction = Box<dyn Fn(&MouseDownEvent, &mut WindowContext)>;

#[derive(IntoElement)]
pub struct Button {
	pub hover_colour: Fill,

	pub icon: &'static str,
	pub click_action: ButtonClickAction,

	pub width: Length,
	pub height: Length
}

impl RenderOnce for Button {
	fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
		div()
			.flex()
			.justify_center()
			.items_center()
			.cursor_pointer()
			.hover(|style| style.bg(self.hover_colour.clone()))
			.on_mouse_down(MouseButton::Left,move |cx, ev| self.click_action.call((cx, ev)))
			.w(self.width)
			.h(self.height)
			.child(SharedString::from(self.icon))
	}
}

