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

use crate::input::Key;

#[derive(Debug, Clone)]
pub enum Command {
    Insert(char),
    Backspace,
}

pub fn key_to_command(key: Key) -> Option<Command> {
    match key {
        Key::Char(c) => Some(Command::Insert(c)),
        Key::Backspace => Some(Command::Backspace),
    }
}