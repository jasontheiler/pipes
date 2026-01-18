pub const PLAIN: CharSet = CharSet {
    horizontal: '─',
    vertical: '│',
    up: '╵',
    up_left: '┘',
    up_right: '└',
    down: '╷',
    down_left: '┐',
    down_right: '┌',
};

pub const ROUNDED: CharSet = CharSet {
    horizontal: '─',
    vertical: '│',
    up: '╵',
    up_left: '╯',
    up_right: '╰',
    down: '╷',
    down_left: '╮',
    down_right: '╭',
};

pub const THICK: CharSet = CharSet {
    horizontal: '━',
    vertical: '┃',
    up: '╹',
    up_left: '┛',
    up_right: '┗',
    down: '╻',
    down_left: '┓',
    down_right: '┏',
};

pub const BLOCK: CharSet = CharSet {
    horizontal: '▀',
    vertical: '█',
    up: '▀',
    up_left: '▀',
    up_right: '▀',
    down: '▄',
    down_left: '█',
    down_right: '█',
};

#[derive(Debug, Clone, Copy)]
pub struct CharSet {
    pub vertical: char,
    pub horizontal: char,
    pub up: char,
    pub up_left: char,
    pub up_right: char,
    pub down: char,
    pub down_left: char,
    pub down_right: char,
}
