use ratatui::{buffer::Buffer, layout::{Margin, Rect}, text::Line, widgets::{Paragraph, Widget}};
use yazi_config::THEME;

use crate::Ctx;

pub(crate) struct Input<'a> {
	cx: &'a Ctx,
}

impl<'a> Input<'a> {
	pub(crate) fn new(cx: &'a Ctx) -> Self { Self { cx } }
}

impl Widget for Input<'_> {
	fn render(self, win: Rect, buf: &mut Buffer) {
		let input = &self.cx.input;
		let mut area = self.cx.mgr.area(input.position);
		area.x = 0;
		area.y = win.height - 1;
		area.height = 1;

		yazi_plugin::elements::Clear::default().render(area, buf);
		let input_value = input.value();
		let input_title = &input.title;
		Paragraph::new(Line::from(format!("{input_title} {input_value}")))
			.style(THEME.input.value)
			.render(area, buf);

		input.render(area.inner(Margin::new(1, 1)), buf);
	}
}
