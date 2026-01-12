use colored::Colorize;

pub struct Toolbar {
    pub active_groups: Vec<usize>,
    pub mastered: usize,
    pub total: usize,
    pub unlocked_groups: usize,
    pub total_groups: usize,
}

impl Toolbar {
    pub fn new(
        active_groups: Vec<usize>,
        mastered: usize,
        total: usize,
        unlocked_groups: usize,
        total_groups: usize,
    ) -> Self {
        Toolbar {
            active_groups,
            mastered,
            total,
            unlocked_groups,
            total_groups,
        }
    }

    pub fn render(&self) {
        let width = 60;
        
        // Top border
        println!("{}", "┌".to_string() + &"─".repeat(width - 2) + "┓");
        
        // Stats line
        let groups_str = format!("📊 Groups: {:?}", self.active_groups);
        let mastery_str = format!("📈 Mastery: {}/{}", self.mastered, self.total);
        let unlocked_str = format!("🎯 Unlocked: {}/{}", self.unlocked_groups, self.total_groups);
        
        let stats_line = format!("{} | {} | {}", groups_str, mastery_str, unlocked_str);
        let padding = if stats_line.len() < width - 4 {
            width - 4 - stats_line.len()
        } else {
            0
        };
        println!("│ {}{} │", stats_line, " ".repeat(padding));
        
        // Shortcuts line
        let shortcuts = "Ctrl+M:Menu | Ctrl+R:Retry | Ctrl+S:Skip | Ctrl+Q:Quit";
        let padding = if shortcuts.len() < width - 4 {
            width - 4 - shortcuts.len()
        } else {
            0
        };
        println!("│ {}{} │", shortcuts.bright_black(), " ".repeat(padding));
        
        // Bottom border
        println!("{}", "└".to_string() + &"─".repeat(width - 2) + "┘");
    }
}
