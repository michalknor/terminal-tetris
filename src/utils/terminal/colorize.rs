#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum TextColors {
	WHITE,
	GRAY,
	RED,
	GREEN,
	YELLOW,
	BLUE,
	PINK,
	CYAN
}


impl TextColors {
    fn get_code(&self) -> i32 {
        match self {
            TextColors::WHITE => 29,
			TextColors::GRAY => 30,
			TextColors::RED => 31,
			TextColors::GREEN => 32,
			TextColors::YELLOW => 33,
			TextColors::BLUE => 34,
			TextColors::PINK => 35,
			TextColors::CYAN => 36,
        }
    }
}



pub fn colorize(text: &str, text_color: TextColors) -> String {
	format!("\x1b[{}m{} \x1b[0m", text_color.get_code(), text)
}