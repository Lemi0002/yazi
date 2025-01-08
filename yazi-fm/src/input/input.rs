use std::ops::Range;

use ratatui::{buffer::Buffer, layout::Rect, text::Line, widgets::{Paragraph, Widget}};
use yazi_config::THEME;
use anyhow::{Result, bail};
use syntect::easy::HighlightLines;
use yazi_core::input::InputMode;

use crate::{Ctx, Term};

pub(crate) struct Input<'a> {
	cx: &'a Ctx,
}

impl<'a> Input<'a> {
	pub(crate) fn new(cx: &'a Ctx) -> Self { Self { cx } }
}

impl Widget for Input<'_> {
	fn render(self, win: Rect, buf: &mut Buffer) {
		let input = &self.cx.input;
		let mut area = self.cx.manager.area(input.position);
		area.x = 0;
		area.y = win.height - 1;
		area.height = 1;

		yazi_plugin::elements::Clear::default().render(area, buf);
		let input_value = input.value();
		let input_title = &input.title;
		Paragraph::new(Line::from(format!("{input_title} {input_value}")))
			.style(THEME.input.value)
			.render(area, buf);

		if let Some(Range { start, end }) = input.selected() {
			let x = win.width.min(area.x + 1 + start);
			let y = win.height.min(area.y + 1);

			buf.set_style(
				Rect { x, y, width: (end - start).min(win.width - x), height: 1.min(win.height - y) },
				THEME.input.selected,
			)
		}

		_ = match input.mode() {
			InputMode::Insert => Term::set_cursor_bar(),
			InputMode::Replace => Term::set_cursor_underscore(),
			_ => Term::set_cursor_block(),
		};
	}
}
