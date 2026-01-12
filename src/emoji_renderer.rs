use colored::Colorize;

pub struct EmojiRenderer {
    supports_color: bool,
}

impl EmojiRenderer {
    pub fn new() -> Self {
        // Check if terminal supports color/emoji
        let supports_color = supports_color::on(supports_color::Stream::Stdout)
            .map(|level| level.has_basic)
            .unwrap_or(false);
        
        EmojiRenderer { supports_color }
    }

    pub fn render(&self, emoji: &str) -> String {
        if self.supports_color {
            // Modern terminals: render emoji with color enhancement
            format!("{}", emoji)
        } else {
            // Fallback for terminals without emoji support
            format!("[{}]", emoji)
        }
    }

    pub fn render_large(&self, emoji: &str) -> String {
        if self.supports_color {
            // Render larger/emphasized emoji for modern terminals
            format!("  {}  ", emoji)
        } else {
            format!(" [{}] ", emoji)
        }
    }
}
