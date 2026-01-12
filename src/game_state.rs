use crate::spaced_repetition::SpacedRepetition;
use crate::word_data::{get_all_words, Word, WordStats};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

const SAVE_FILE: &str = "progress.json";

#[derive(Debug, Serialize, Deserialize)]
pub struct GameState {
    pub unlocked_groups: Vec<usize>,
    pub selected_groups: Option<Vec<usize>>, // None = use unlocked_groups, Some = manual selection
    pub stats: Vec<WordStats>,
    pub placement_test_progress: Option<usize>, // Current group being tested (None = not in test mode)
}

impl GameState {
    pub fn new() -> Self {
        let words = get_all_words();
        GameState {
            unlocked_groups: vec![1], // Start with group 1 unlocked
            selected_groups: None, // Use unlocked groups by default
            stats: (0..words.len()).map(|id| WordStats::new(id)).collect(),
            placement_test_progress: None, // Not in test mode
        }
    }

    pub fn load() -> Self {
        let save_path = get_save_path();
        
        if save_path.exists() {
            match fs::read_to_string(&save_path) {
                Ok(content) => {
                    match serde_json::from_str(&content) {
                        Ok(state) => return state,
                        Err(e) => eprintln!("Failed to parse save file: {}", e),
                    }
                }
                Err(e) => eprintln!("Failed to read save file: {}", e),
            }
        }
        
        // If loading fails or file doesn't exist, create new state
        GameState::new()
    }

    pub fn save(&self) {
        let save_path = get_save_path();
        
        match serde_json::to_string_pretty(self) {
            Ok(json) => {
                if let Err(e) = fs::write(&save_path, json) {
                    eprintln!("Failed to save progress: {}", e);
                } else {
                    println!("Progress saved!");
                }
            }
            Err(e) => eprintln!("Failed to serialize game state: {}", e),
        }
    }

    pub fn get_available_words(&self) -> Vec<(usize, Word)> {
        let all_words = get_all_words();
        let active_groups = self.selected_groups.as_ref().unwrap_or(&self.unlocked_groups);
        
        all_words
            .into_iter()
            .enumerate()
            .filter(|(_, word)| active_groups.contains(&word.group))
            .collect()
    }

    pub fn get_active_groups(&self) -> &Vec<usize> {
        self.selected_groups.as_ref().unwrap_or(&self.unlocked_groups)
    }

    pub fn set_selected_groups(&mut self, groups: Vec<usize>) {
        self.selected_groups = Some(groups);
    }

    pub fn clear_selected_groups(&mut self) {
        self.selected_groups = None;
    }

    pub fn check_and_unlock_next_group(&mut self, sr: &SpacedRepetition) {
        let available_words = self.get_available_words();
        let available_ids: Vec<usize> = available_words.iter().map(|(id, _)| *id).collect();
        
        if available_ids.is_empty() {
            return;
        }
        
        // Count mature words (5+ correct answers)
        let mature_count = available_ids.iter()
            .filter(|&&id| {
                sr.get_stats().get(id)
                    .map(|s| s.correct_count >= 5)
                    .unwrap_or(false)
            })
            .count();
        
        // Require 80% mastery
        let required_mature = (available_ids.len() as f32 * 0.8).ceil() as usize;
        
        if mature_count >= required_mature {
            // Unlock next group
            let max_unlocked = *self.unlocked_groups.iter().max().unwrap_or(&1);
            let next_group = max_unlocked + 1;
            
            // Check if next group exists
            let all_words = get_all_words();
            let next_group_exists = all_words.iter().any(|w| w.group == next_group);
            
            if next_group_exists && !self.unlocked_groups.contains(&next_group) {
                self.unlocked_groups.push(next_group);
                println!("\n🎉 Mastery Achieved! {} words mastered! Unlocked Group {}! 🎉\n", 
                    mature_count, next_group);
            }
        }
    }

    pub fn get_mastery_progress(&self, sr: &SpacedRepetition) -> (usize, usize) {
        let available_words = self.get_available_words();
        let available_ids: Vec<usize> = available_words.iter().map(|(id, _)| *id).collect();
        
        if available_ids.is_empty() {
            return (0, 0);
        }
        
        let mature_count = available_ids.iter()
            .filter(|&&id| {
                sr.get_stats().get(id)
                    .map(|s| s.correct_count >= 5)
                    .unwrap_or(false)
            })
            .count();
        
        let required = (available_ids.len() as f32 * 0.8).ceil() as usize;
        (mature_count, required)
    }
}

fn get_save_path() -> PathBuf {
    let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push("emoji_japanese_learner");
    
    // Create directory if it doesn't exist
    if !path.exists() {
        let _ = fs::create_dir_all(&path);
    }
    
    path.push(SAVE_FILE);
    path
}
