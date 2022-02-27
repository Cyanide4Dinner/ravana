pub struct Color {
    red: u8,
    green: u8,
    blue: u8
}

pub struct Theme {
    highlight-fg: Color,
    highlight-bg: Color
}

pub struct TuiPrefs {
    theme: Theme
}
