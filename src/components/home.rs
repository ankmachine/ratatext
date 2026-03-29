use super::Component;
use super::process_data::DataProcessor;
use color_eyre::Result;
use crossterm::event::KeyCode;
use fuzzy_matcher::{FuzzyMatcher, skim::SkimMatcherV2};
use ratatui::{prelude::*, widgets::*};
use tokio::sync::mpsc::UnboundedSender;

use crate::{action::Action, config::Config};

#[derive(Default)]
pub struct Home {
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,
    input: String,
    character_index: usize,
    search_list: Vec<String>,
    data_process: DataProcessor,
    is_loading: bool,
    filtered_list: Vec<String>,
    fuzzy_matcher: SkimMatcherV2,
}

impl Home {
    pub fn new() -> Self {
        let initial_list = vec!["Loading ...".to_string()];
        Home {
            search_list: initial_list.clone(),
            filtered_list: initial_list,
            fuzzy_matcher: SkimMatcherV2::default(),
            data_process: DataProcessor::new(),
            is_loading: true,
            ..Default::default()
        }
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
        self.update_filtered_list();
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
            self.update_filtered_list();
        }
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input.chars().count())
    }

    fn _reset_cursor(&mut self) {
        self.character_index = 0;
    }

    pub fn update_search_list(&mut self, search_list: Vec<String>) {
        self.search_list = search_list;
        self.is_loading = false;
        self.update_filtered_list();
    }

    fn update_filtered_list(&mut self) {
        if self.input.trim().is_empty() {
            self.filtered_list = self.search_list.clone();
        } else {
            let mut scored_item: Vec<(String, f64)> = Vec::new();

            for item in &self.search_list {
                if let Some(score) = self.fuzzy_matcher.fuzzy_match(item, &self.input) {
                    scored_item.push((item.clone(), score as f64));
                }
            }

            // sort
            scored_item.sort_by(|a, b| b.1.total_cmp(&a.1));
            self.filtered_list = scored_item.into_iter().map(|(item, _)| item).collect();
        }
    }
}

impl Component for Home {
    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
        self.command_tx = Some(tx.clone());

        let processor = self.data_process.clone();
        tokio::spawn(async move {
            let search_list = processor.fetch_list_safe().await;
            let _ = tx.send(Action::DataLoaded(search_list));
        });
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
            Action::DataLoaded(search_list) => {
                self.update_search_list(search_list);
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

    fn draw(&mut self, frame: &mut Frame, _area: Rect) -> Result<()> {
        let vertical = Layout::vertical([Constraint::Percentage(15), Constraint::Percentage(85)]);

        let [input_area, list_area] = vertical.areas(frame.area());
        let input = Paragraph::new(self.input.as_str()).block(Block::bordered().title("Input"));
        frame.render_widget(input, input_area);
        let cursor_x = input_area.x + 1 + self.character_index as u16;
        let cursor_y = input_area.y + 1;
        if cursor_x > input_area.right() - 1 {
            frame.set_cursor_position((cursor_x, cursor_y));
        }
        let list = List::new(
            self.filtered_list
                .iter()
                .map(|s| s.as_str())
                .collect::<Vec<_>>(),
        )
        .block(Block::bordered().title("List"));
        frame.render_widget(list, list_area);
        Ok(())
    }
}
