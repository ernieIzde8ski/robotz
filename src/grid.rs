/// Minimum width of the terminal.
pub const MIN_COLS: u16 = 80;
/// Minimum height of the terminal.
pub const MIN_ROWS: u16 = 24;

pub const OUTER_GRID_LEFT_COLUMN: u16 = 0;
pub const OUTER_GRID_RIGHT_COLUMN: u16 = 60;
pub const OUTER_GRID_BOTTOM_ROW: u16 = 24;
pub const OUTER_GRID_TOP_ROW: u16 = 0;

pub const INNER_GRID_LEFT_COLUMN: u16 = OUTER_GRID_LEFT_COLUMN + 1;
pub const INNER_GRID_RIGHT_COLUMN: u16 = OUTER_GRID_RIGHT_COLUMN - 1;
pub const INNER_GRID_BOTTOM_ROW: u16 = OUTER_GRID_BOTTOM_ROW - 1;
pub const INNER_GRID_TOP_ROW: u16 = OUTER_GRID_TOP_ROW + 1;
pub const INNER_GRID_COLUMN_TOTAL: u16 = INNER_GRID_RIGHT_COLUMN - INNER_GRID_LEFT_COLUMN;
pub const INNER_GRID_ROW_TOTAL: u16 = INNER_GRID_BOTTOM_ROW - INNER_GRID_TOP_ROW;

lazy_static::lazy_static! {
    pub static ref ASCII_TOP_ROW: String = {
        format!("{}{}{}\r\n", '┌', "─".repeat(INNER_GRID_COLUMN_TOTAL as usize), '┐')
    };

    pub static ref ASCII_MIDDLE_ROW: String = {
        format!("{}{}{}\r\n", '│', " ".repeat(INNER_GRID_COLUMN_TOTAL as usize), '│')
    };

    pub static ref ASCII_BOTTOM_ROW: String = {
        format!("{}{}{}\r\n", '└', "─".repeat((INNER_GRID_COLUMN_TOTAL) as usize), '┘')
    };
}
