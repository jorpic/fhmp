use crate::config::Config;
use crate::issues::Issues;
use ratatui::widgets::TableState;

pub struct AppState {
    pub config: Config,
    pub issues: Issues,
    pub table_state: TableState,
}
