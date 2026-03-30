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
    widget::{container, text},
};

use oxide_core::editor::Editor as CoreEditor;

use crate::app::message::Message;

pub struct EditorView<'a> {
    editor: &'a CoreEditor,
}

impl<'a> EditorView<'a> {
    pub fn new(editor: &'a CoreEditor) -> Self {
        Self { editor }
    }

    pub fn view(self) -> Element<'a, Message> {
        // for now lets just render the raw buffer
        // later we will implement lines, cursor, and whatever else
        println!("{}", self.editor.buffer.as_str());
        container(text(self.editor.buffer.as_str()))
            .padding(10)
            .into()
    }
}
