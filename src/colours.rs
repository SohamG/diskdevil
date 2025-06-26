// courtesy of ChatGPT

pub const RESET: &'static str = "\x1b[0m";
pub const BOLD: &'static str = "\x1b[1m";
pub const DIM: &'static str = "\x1b[2m";
pub const ITALIC: &'static str = "\x1b[3m";
pub const UNDERLINE: &'static str = "\x1b[4m";
pub const BLINK: &'static str = "\x1b[5m";
pub const REVERSE: &'static str = "\x1b[7m";
pub const HIDDEN: &'static str = "\x1b[8m";
pub const STRIKETHROUGH: &'static str = "\x1b[9m";

/// Foreground 8 basic colors
pub const FG_BLACK: &'static str = "\x1b[30m";
pub const FG_RED: &'static str = "\x1b[31m";
pub const FG_GREEN: &'static str = "\x1b[32m";
pub const FG_YELLOW: &'static str = "\x1b[33m";
pub const FG_BLUE: &'static str = "\x1b[34m";
pub const FG_MAGENTA: &'static str = "\x1b[35m";
pub const FG_CYAN: &'static str = "\x1b[36m";
pub const FG_WHITE: &'static str = "\x1b[37m";

/// Bright foreground colors
pub const FG_BRIGHT_BLACK: &'static str = "\x1b[90m";
pub const FG_BRIGHT_RED: &'static str = "\x1b[91m";
pub const FG_BRIGHT_GREEN: &'static str = "\x1b[92m";
pub const FG_BRIGHT_YELLOW: &'static str = "\x1b[93m";
pub const FG_BRIGHT_BLUE: &'static str = "\x1b[94m";
pub const FG_BRIGHT_MAGENTA: &'static str = "\x1b[95m";
pub const FG_BRIGHT_CYAN: &'static str = "\x1b[96m";
pub const FG_BRIGHT_WHITE: &'static str = "\x1b[97m";

/// Background 8 basic colors
pub const BG_BLACK: &'static str = "\x1b[40m";
pub const BG_RED: &'static str = "\x1b[41m";
pub const BG_GREEN: &'static str = "\x1b[42m";
pub const BG_YELLOW: &'static str = "\x1b[43m";
pub const BG_BLUE: &'static str = "\x1b[44m";
pub const BG_MAGENTA: &'static str = "\x1b[45m";
pub const BG_CYAN: &'static str = "\x1b[46m";
pub const BG_WHITE: &'static str = "\x1b[47m";

/// Bright background colors
pub const BG_BRIGHT_BLACK: &'static str = "\x1b[100m";
pub const BG_BRIGHT_RED: &'static str = "\x1b[101m";
pub const BG_BRIGHT_GREEN: &'static str = "\x1b[102m";
pub const BG_BRIGHT_YELLOW: &'static str = "\x1b[103m";
pub const BG_BRIGHT_BLUE: &'static str = "\x1b[104m";
pub const BG_BRIGHT_MAGENTA: &'static str = "\x1b[105m";
pub const BG_BRIGHT_CYAN: &'static str = "\x1b[106m";
pub const BG_BRIGHT_WHITE: &'static str = "\x1b[107m";

/// 256-color foreground (use {} for color index 0-255)
pub const FG_COLOR_256: &'static str = "\x1b[38;5;{color}m";

/// 256-color background
pub const BG_COLOR_256: &'static str = "\x1b[48;5;{color}m";

/// Cursor movement
pub const CURSOR_UP: &'static str = "\x1b[{n}A";
pub const CURSOR_DOWN: &'static str = "\x1b[{n}B";
pub const CURSOR_FORWARD: &'static str = "\x1b[{n}C";
pub const CURSOR_BACK: &'static str = "\x1b[{n}D";
pub const CURSOR_NEXT_LINE: &'static str = "\x1b[{n}E";
pub const CURSOR_PREV_LINE: &'static str = "\x1b[{n}F";
pub const CURSOR_COLUMN: &'static str = "\x1b[{n}G";
pub const CURSOR_POSITION: &'static str = "\x1b[{row};{col}H";
pub const CURSOR_SAVE: &'static str = "\x1b[s";
pub const CURSOR_RESTORE: &'static str = "\x1b[u";
pub const CURSOR_HIDE: &'static str = "\x1b[?25l";
pub const CURSOR_SHOW: &'static str = "\x1b[?25h";

/// Screen clearing
pub const CLEAR_SCREEN: &'static str = "\x1b[2J";
pub const CLEAR_LINE: &'static str = "\x1b[2K";
pub const CLEAR_BELOW: &'static str = "\x1b[0J";
pub const CLEAR_ABOVE: &'static str = "\x1b[1J";
pub const CLEAR_LINE_BEFORE: &'static str = "\x1b[1K";
pub const CLEAR_LINE_AFTER: &'static str = "\x1b[0K";

/// Scrolling
pub const SCROLL_UP: &'static str = "\x1b[S";
pub const SCROLL_DOWN: &'static str = "\x1b[T";

/// Alternate screen buffer
pub const ENTER_ALTERNATE_SCREEN: &'static str = "\x1b[?1049h";
pub const EXIT_ALTERNATE_SCREEN: &'static str = "\x1b[?1049l";

/// Mouse input (xterm mouse tracking)
pub const ENABLE_MOUSE_TRACKING: &'static str = "\x1b[?1000h";
pub const DISABLE_MOUSE_TRACKING: &'static str = "\x1b[?1000l";
pub const ENABLE_SGR_MOUSE: &'static str = "\x1b[?1006h";
pub const DISABLE_SGR_MOUSE: &'static str = "\x1b[?1006l";

/// Bracketed paste mode
pub const ENABLE_BRACKETED_PASTE: &'static str = "\x1b[?2004h";
pub const DISABLE_BRACKETED_PASTE: &'static str = "\x1b[?2004l";
