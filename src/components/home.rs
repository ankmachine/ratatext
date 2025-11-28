use color_eyre::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use futures::future::ok;
use ratatui::{prelude::*, widgets::*};
use tokio::sync::mpsc::UnboundedSender;

use super::Component;
use crate::{action::Action, config::Config, tui::Event};

#[derive(Default)]
pub struct Home {
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,
    input: String,
    character_index: usize,
}

impl Home {
    pub fn new() -> Self {
        Self::default()
    }
    fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.character_index.saturating_sub(1);
        self.character_index = self.clamp_cursor(cursor_moved_left);
    }

    fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.character_index.saturating_add(1);
        self.character_index = self.clamp_cursor(cursor_moved_right);
    }

    fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.input.insert(index, new_char);
        self.move_cursor_right();
    }
    fn byte_index(&self) -> usize {
        self.input
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.character_index)
            .unwrap_or(self.input.len())
    }
    fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.character_index != 0;
        if is_not_cursor_leftmost {
            // Method "remove" is not used on the saved text for deleting the selected char.
            // Reason: Using remove on String works on bytes instead of the chars.
            // Using remove would require special care because of char boundaries.

            let current_index = self.character_index;
            let from_left_to_current_index = current_index - 1;

            // Getting all characters before the selected character.
            let before_char_to_delete = self.input.chars().take(from_left_to_current_index);
            // Getting all characters after selected character.
            let after_char_to_delete = self.input.chars().skip(current_index);

            // Put all characters together except the selected one.
            // By leaving the selected one out, it is forgotten and therefore deleted.
            self.input = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input.chars().count())
    }

    fn reset_cursor(&mut self) {
        self.character_index = 0;
    }
}

impl Component for Home {
    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
        self.command_tx = Some(tx);
        Ok(())
    }

    fn register_config_handler(&mut self, config: Config) -> Result<()> {
        self.config = config;
        Ok(())
    }

    fn update(&mut self, action: Action) -> Result<Option<Action>> {
        match action {
            Action::Tick => {

                // add any logic here that should run on every tick
            }
            Action::Render => {
                // add any logic here that should run on every render
            }
            _ => {}
        }
        Ok(None)
    }

    fn handle_key_event(&mut self, key: crossterm::event::KeyEvent) -> Result<Option<Action>> {
        match key.code {
            KeyCode::Char(to_insert) => {
                self.enter_char(to_insert);
                Ok(None)
            }
            // KeyCode::Enter => self.submit_message(),
            KeyCode::Backspace => {
                self.delete_char();
                Ok(None)
            }
            KeyCode::Left => {
                self.move_cursor_left();
                Ok(None)
            }
            KeyCode::Right => {
                self.move_cursor_right();
                Ok(None)
            }
            _ => Ok(None),
        }
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        let vertical = Layout::vertical([Constraint::Percentage(15), Constraint::Percentage(85)]);

        let [input_area, list_area] = vertical.areas(frame.area());
        let input = Paragraph::new(self.input.as_str()).block(Block::bordered().title("Input"));
        frame.render_widget(input, input_area);
        let list = Paragraph::new("hello world").block(Block::bordered().title("List"));
        frame.render_widget(list, list_area);
        Ok(())
    }
}
