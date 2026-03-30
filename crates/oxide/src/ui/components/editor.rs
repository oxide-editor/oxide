/*
 * Oxide - A opinionated code editor.
 * Copyright (C) 2026 Oxide Team
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published
 * by the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use iced::{
    Element,
    widget::{container, text_editor},
};

use crate::app::message::Message;

pub struct EditorView<'a> {
    content: &'a text_editor::Content,
}

impl<'a> EditorView<'a> {
    pub fn new(content: &'a text_editor::Content) -> Self {
        Self { content }
    }

    pub fn view(self) -> Element<'a, Message> {
        container(
            text_editor(self.content)
                .on_action(Message::Edit)
                .style(|theme, status| {
                    let mut style = iced::widget::text_editor::default(theme, status);
                    style.border = iced::Border::default();
                    style
                }),
        )
        .into()
    }
}
