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

use iced::event;
use iced::keyboard::{Event as KeyEvent, Key as IcedKey};
use oxide_core::input::{Key, key_to_command};

use crate::app::message::Message;

pub fn keyboard_subscription() -> iced::Subscription<Message> {
    iced::event::listen().filter_map(|event| {
        if let event::Event::Keyboard(KeyEvent::KeyPressed { key, .. }) = event {
            let key = match key {
                IcedKey::Character(s) => {
                    if let Some(c) = s.chars().next() {
                        Key::Char(c)
                    } else {
                        return None;
                    }
                }
                IcedKey::Named(iced::keyboard::key::Named::Backspace) => Key::Backspace,
                _ => return None,
            };

            key_to_command(key).map(Message::Input)
        } else {
            None
        }
    })
}
