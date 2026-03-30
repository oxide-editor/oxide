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

use iced::{Element, Task};
use oxide_core::editor::Editor;

use crate::{app::message::Message, ui::components::editor::EditorView};

pub struct OxideApp {
    editor: Editor,
}

impl Default for OxideApp {
    fn default() -> Self {
        Self {
            editor: Editor::default(),
        }
    }
}

impl OxideApp {
    pub fn subscription(&self) -> iced::Subscription<Message> {
        crate::app::subscriptions::keyboard_subscription()
    }

    pub fn view(&self) -> Element<'_, Message> {
        EditorView::new(&self.editor).view()
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        let Message::Input(cmd) = message;
        self.editor.apply_command(cmd);

        Task::none()
    }
}
