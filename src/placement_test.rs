use crate::game_state::GameState;
use crate::spaced_repetition::SpacedRepetition;
use crate::word_data::{get_all_words, get_group_info};
use colored::Colorize;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PlacementTestState {
    pub current_group: usize,
    pub completed_groups: Vec<usize>,
}

impl PlacementTestState {
    pub fn new() -> Self {
        PlacementTestState {
            current_group: 1,
            completed_groups: Vec::new(),
        }
    }

    pub fn mark_group_completed(&mut self, group: usize) {
        if !self.completed_groups.contains(&group) {
            self.completed_groups.push(group);
        }
        self.current_group = group + 1;
    }

    pub fn is_complete(&self) -> bool {
        let group_info = get_group_info();
        self.current_group > group_info.len()
    }
}

pub fn mark_group_as_mastered(
    game_state: &mut GameState,
    sr: &mut SpacedRepetition,
    group: usize,
) {
    let all_words = get_all_words();
    
    // Find all words in this group
    let group_word_ids: Vec<usize> = all_words
        .iter()
        .enumerate()
        .filter(|(_, word)| word.group == group)
        .map(|(id, _)| id)
        .collect();
    
    // Mark each word as mastered (5 correct answers)
    for word_id in group_word_ids {
        if let Some(stats) = sr.get_stats_mut().get_mut(word_id) {
            stats.correct_count = 5;
            stats.incorrect_count = 0;
            stats.last_seen = Some(
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            );
        }
    }
    
    // Unlock the group
    if !game_state.unlocked_groups.contains(&group) {
        game_state.unlocked_groups.push(group);
    }
    
    println!("{}", format!("✓ Group {} marked as mastered!", group).bright_green().bold());
}
