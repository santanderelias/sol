use crate::word_data::WordStats;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct SpacedRepetition {
    stats: Vec<WordStats>,
}

impl SpacedRepetition {
    pub fn new(word_count: usize) -> Self {
        let stats = (0..word_count).map(|id| WordStats::new(id)).collect();
        SpacedRepetition { stats }
    }

    pub fn from_stats(stats: Vec<WordStats>) -> Self {
        SpacedRepetition { stats }
    }

    pub fn get_stats(&self) -> &Vec<WordStats> {
        &self.stats
    }

    pub fn get_stats_mut(&mut self) -> &mut Vec<WordStats> {
        &mut self.stats
    }

    /// Select the next word to show based on SSRS algorithm
    /// Priority is given to:
    /// 1. Words never seen before
    /// 2. Words with lower ease factor (struggled with)
    /// 3. Words due for review based on interval
    pub fn select_next_word(&self, available_word_ids: &[usize]) -> Option<usize> {
        if available_word_ids.is_empty() {
            return None;
        }

        let current_time = get_current_timestamp();
        let mut best_word_id = available_word_ids[0];
        let mut best_priority = f32::MIN;

        for &word_id in available_word_ids {
            if word_id >= self.stats.len() {
                continue;
            }

            let stat = &self.stats[word_id];
            let priority = self.calculate_priority(stat, current_time);

            if priority > best_priority {
                best_priority = priority;
                best_word_id = word_id;
            }
        }

        Some(best_word_id)
    }

    fn calculate_priority(&self, stat: &WordStats, current_time: u64) -> f32 {
        // Never seen before gets highest priority
        if stat.last_seen.is_none() {
            return 1000.0;
        }

        let last_seen = stat.last_seen.unwrap();
        let time_since_seen = (current_time - last_seen) as f32;
        
        // Calculate how overdue this word is
        let interval_seconds = stat.interval as f32 * 86400.0; // Convert days to seconds
        let overdue_factor = if interval_seconds > 0.0 {
            time_since_seen / interval_seconds
        } else {
            time_since_seen / 60.0 // If interval is 0, use 1 minute as base
        };

        // Lower ease factor = struggled more = higher priority
        let difficulty_factor = 3.0 - stat.ease_factor.min(2.5);
        
        // Combine factors: overdue words and difficult words get higher priority
        overdue_factor * 10.0 + difficulty_factor * 5.0
    }

    /// Update statistics after user answers
    pub fn update_after_answer(&mut self, word_id: usize, correct: bool) {
        if word_id >= self.stats.len() {
            return;
        }

        let stat = &mut self.stats[word_id];
        let current_time = get_current_timestamp();
        stat.last_seen = Some(current_time);

        if correct {
            stat.correct_count += 1;
            
            // SM-2 algorithm: increase ease factor for correct answers
            stat.ease_factor = (stat.ease_factor + 0.1).min(3.0);
            
            // Increase interval
            if stat.interval == 0 {
                stat.interval = 1;
            } else {
                stat.interval = (stat.interval as f32 * stat.ease_factor) as u32;
            }
        } else {
            stat.incorrect_count += 1;
            
            // Decrease ease factor for incorrect answers
            stat.ease_factor = (stat.ease_factor - 0.2).max(1.3);
            
            // Reset interval to show again soon
            stat.interval = 0;
        }
    }

    pub fn get_total_correct_for_groups(&self, word_ids: &[usize]) -> u32 {
        word_ids
            .iter()
            .filter_map(|&id| self.stats.get(id))
            .map(|stat| stat.correct_count)
            .sum()
    }
}

fn get_current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
