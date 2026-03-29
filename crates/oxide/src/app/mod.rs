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

pub mod message;

use iced::{Element, Task, widget::text};
use message::Message;

#[derive(Default)]
pub struct OxideApp {
    // placeholder for future state, e.g., text buffer, settings
}

impl OxideApp {
    pub fn view(&self) -> Element<'_, Message> {
        text("Hello, world!").into()
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        println!("{:?}", message); // just to make rust shutup about it being unused
        Task::none()
    }
}
