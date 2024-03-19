use ratatui::widgets::TableState;
use crate::config::Config;
use crate::issues::Issues;

pub struct AppState {
    pub config: Config,
    pub issues: Issues,
    pub table_state: TableState,
}
