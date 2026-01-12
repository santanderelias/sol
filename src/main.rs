mod word_data;
mod spaced_repetition;
mod input_validator;
mod game_state;
mod emoji_renderer;
mod toolbar;
mod placement_test;

use colored::Colorize;
use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use emoji_renderer::EmojiRenderer;
use game_state::GameState;
use input_validator::is_correct_answer;
use placement_test::{mark_group_as_mastered, PlacementTestState};
use spaced_repetition::SpacedRepetition;
use toolbar::Toolbar;
use word_data::{get_all_words, get_group_info};
use std::io::{self, Write};
use std::time::Duration;

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}

enum InputResult {
    Answer(String),
    Quit,
    Retry,
    Skip,
    Menu,
}

fn read_input_with_shortcuts() -> io::Result<InputResult> {
    let mut buffer = String::new();
    
    print!("{} ", "Your answer:".bright_blue());
    io::stdout().flush()?;
    
    enable_raw_mode()?;
    
    loop {
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key_event) = event::read()? {
                match (key_event.code, key_event.modifiers) {
                    (KeyCode::Char('q'), KeyModifiers::CONTROL) => {
                        disable_raw_mode()?;
                        println!();
                        return Ok(InputResult::Quit);
                    }
                    (KeyCode::Char('r'), KeyModifiers::CONTROL) => {
                        disable_raw_mode()?;
                        println!();
                        return Ok(InputResult::Retry);
                    }
                    (KeyCode::Char('s'), KeyModifiers::CONTROL) => {
                        disable_raw_mode()?;
                        println!();
                        return Ok(InputResult::Skip);
                    }
                    (KeyCode::Char('m'), KeyModifiers::CONTROL) => {
                        disable_raw_mode()?;
                        println!();
                        return Ok(InputResult::Menu);
                    }
                    (KeyCode::Enter, _) => {
                        disable_raw_mode()?;
                        println!();
                        return Ok(InputResult::Answer(buffer));
                    }
                    (KeyCode::Char(c), _) => {
                        buffer.push(c);
                        print!("{}", c);
                        io::stdout().flush()?;
                    }
                    (KeyCode::Backspace, _) => {
                        if buffer.pop().is_some() {
                            print!("\x08 \x08");
                            io::stdout().flush()?;
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}

fn show_group_selection_menu(game_state: &mut GameState) {
    clear_screen();
    
    println!("{}", "=".repeat(60).bright_cyan());
    println!("{}", "📚 Group Selection Menu 📚".bright_yellow().bold());
    println!("{}", "=".repeat(60).bright_cyan());
    println!();
    
    let group_info = get_group_info();
    let unlocked = &game_state.unlocked_groups;
    
    println!("{}", "Available Groups:".bright_blue().bold());
    println!();
    
    for info in &group_info {
        let status = if unlocked.contains(&info.id) {
            "✓ Unlocked".green()
        } else {
            "🔒 Locked".bright_black()
        };
        
        println!("  {}. {} - {} {}", 
            info.id.to_string().bright_cyan(),
            info.name.bright_white().bold(),
            info.description.bright_black(),
            status
        );
    }
    
    println!();
    println!("{}", "Enter group numbers separated by spaces (e.g., '1 2 3')".bright_blue());
    println!("{}", "Or press Enter to use all unlocked groups".bright_blue());
    println!("{}", "Type 'back' to return to the game".bright_blue());
    println!();
    
    loop {
        print!("{} ", "Your selection:".bright_blue());
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        if input.eq_ignore_ascii_case("back") {
            return;
        }
        
        if input.is_empty() {
            game_state.clear_selected_groups();
            println!("{}", "Using all unlocked groups!".green());
            std::thread::sleep(std::time::Duration::from_secs(1));
            return;
        }
        
        let selected: Result<Vec<usize>, _> = input
            .split_whitespace()
            .map(|s| s.parse::<usize>())
            .collect();
        
        match selected {
            Ok(groups) => {
                let invalid: Vec<_> = groups.iter()
                    .filter(|g| !unlocked.contains(g))
                    .collect();
                
                if !invalid.is_empty() {
                    println!("{}", format!("Error: Groups {:?} are locked!", invalid).red());
                    println!();
                    continue;
                }
                
                if groups.is_empty() {
                    println!("{}", "Error: Please select at least one group!".red());
                    println!();
                    continue;
                }
                
                game_state.set_selected_groups(groups.clone());
                println!("{}", format!("Selected groups: {:?}", groups).green());
                std::thread::sleep(std::time::Duration::from_secs(1));
                return;
            }
            Err(_) => {
                println!("{}", "Error: Invalid input! Please enter numbers separated by spaces.".red());
                println!();
            }
        }
    }
}

fn run_placement_test(game_state: &mut GameState, sr: &mut SpacedRepetition, emoji_renderer: &EmojiRenderer) -> bool {
    let group_to_test = game_state.placement_test_progress.unwrap_or(1);
    let all_words = get_all_words();
    let group_info = get_group_info();
    
    if group_to_test > group_info.len() {
        println!("{}", "All groups tested!".bright_green().bold());
        game_state.placement_test_progress = None;
        return true;
    }
    
    let group_words: Vec<_> = all_words
        .iter()
        .enumerate()
        .filter(|(_, w)| w.group == group_to_test)
        .collect();
    
    if group_words.is_empty() {
        game_state.placement_test_progress = Some(group_to_test + 1);
        return false;
    }
    
    clear_screen();
    println!("{}", "=".repeat(60).bright_cyan());
    println!("{}", format!("📝 Placement Test - Group {} ({})", 
        group_to_test, 
        group_info.get(group_to_test - 1).map(|g| g.name.as_str()).unwrap_or("Unknown")
    ).bright_yellow().bold());
    println!("{}", "=".repeat(60).bright_cyan());
    println!();
    println!("{}", format!("Testing {} words. Get ALL correct to skip this group!", group_words.len()).bright_blue());
    println!();
    println!("Press Enter to start...");
    let mut _temp = String::new();
    io::stdin().read_line(&mut _temp).unwrap();
    
    let mut all_correct = true;
    
    for (word_id, word) in &group_words {
        clear_screen();
        println!("{}", "=".repeat(60).bright_cyan());
        println!("{}", format!("📝 Placement Test - Group {}", group_to_test).bright_yellow().bold());
        println!("{}", "=".repeat(60).bright_cyan());
        println!();
        
        println!("{} {}", "Emoji:".bright_blue().bold(), emoji_renderer.render_large(&word.emoji));
        println!();
        
        match read_input_with_shortcuts() {
            Ok(InputResult::Answer(answer)) => {
                let correct = is_correct_answer(&answer, &word.japanese, &word.romanji, word.kanji.as_deref());
                
                if !correct {
                    println!();
                    println!("{}", "Incorrect!".bright_red().bold());
                    print!("  Correct: {} ({})", word.japanese.bright_cyan(), word.romanji.bright_black());
                    if let Some(ref kanji) = word.kanji {
                        print!(" or {}", kanji.bright_cyan());
                    }
                    println!();
                    println!();
                    println!("{}", format!("Test failed. You'll start learning from Group {}.", group_to_test).yellow());
                    all_correct = false;
                    break;
                }
            }
            Ok(InputResult::Quit) => {
                println!("{}", "Test paused. Progress saved.".yellow());
                return true;
            }
            _ => {
                println!("{}", "Invalid input during test.".red());
                all_correct = false;
                break;
            }
        }
    }
    
    if all_correct {
        println!();
        println!("{}", format!("✓ Perfect! Group {} mastered!", group_to_test).bright_green().bold());
        mark_group_as_mastered(game_state, sr, group_to_test);
        game_state.placement_test_progress = Some(group_to_test + 1);
        println!();
        println!("Press Enter to continue...");
        let mut _temp = String::new();
        io::stdin().read_line(&mut _temp).unwrap();
        return false;
    } else {
        game_state.placement_test_progress = None;
        println!();
        println!("Press Enter to start learning...");
        let mut _temp = String::new();
        io::stdin().read_line(&mut _temp).unwrap();
        return true;
    }
}

fn show_startup_menu(game_state: &GameState) -> u8 {
    clear_screen();
    
    println!("{}", "=".repeat(60).bright_cyan());
    println!("{}", "🎌 Japanese Emoji Learning Game 🎌".bright_yellow().bold());
    println!("{}", "=".repeat(60).bright_cyan());
    println!();
    
    println!("1. {} - Start learning", "Start Learning".bright_green());
    println!("2. {} - Test your knowledge and skip mastered content", "Placement Test".bright_yellow());
    
    if game_state.placement_test_progress.is_some() {
        println!("3. {} - Resume from Group {}", 
            "Continue Test".bright_cyan(),
            game_state.placement_test_progress.unwrap()
        );
    }
    
    println!();
    print!("{} ", "Your choice (1-3):".bright_blue());
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    input.trim().parse().unwrap_or(1)
}

fn main() {
    let emoji_renderer = EmojiRenderer::new();
    let mut game_state = GameState::load();
    let mut sr = SpacedRepetition::from_stats(game_state.stats.clone());
    
    // Show startup menu
    let choice = show_startup_menu(&game_state);
    
    match choice {
        2 => {
            game_state.placement_test_progress = Some(1);
        }
        3 if game_state.placement_test_progress.is_some() => {
            // Continue existing test
        }
        _ => {
            game_state.placement_test_progress = None;
        }
    }
    
    // Run placement test if in test mode
    while game_state.placement_test_progress.is_some() {
        if run_placement_test(&mut game_state, &mut sr, &emoji_renderer) {
            break;
        }
    }
    
    // Main learning loop
    loop {
        let available_words = game_state.get_available_words();
        
        if available_words.is_empty() {
            clear_screen();
            println!("{}", "No words available! Use Ctrl+M to select groups.".red());
            println!();
            println!("Press Enter to open menu...");
            let mut _temp = String::new();
            io::stdin().read_line(&mut _temp).unwrap();
            show_group_selection_menu(&mut game_state);
            continue;
        }

        let available_ids: Vec<usize> = available_words.iter().map(|(id, _)| *id).collect();
        
        let word_id = match sr.select_next_word(&available_ids) {
            Some(id) => id,
            None => {
                println!("{}", "No more words!".yellow());
                break;
            }
        };

        let (_, word) = &available_words.iter().find(|(id, _)| *id == word_id).unwrap();

        clear_screen();
        
        // Show toolbar
        let active_groups = game_state.get_active_groups().clone();
        let (mature_count, required) = game_state.get_mastery_progress(&sr);
        let toolbar = Toolbar::new(
            active_groups,
            mature_count,
            available_ids.len(),
            game_state.unlocked_groups.len(),
            get_group_info().len(),
        );
        toolbar.render();
        println!();
        
        // Check if new word
        let is_new_word = sr.get_stats().get(word_id).map(|s| s.is_new()).unwrap_or(true);

        if is_new_word {
            println!("{}", "📖 New Word - Learning Mode".bright_yellow().bold());
            println!();
            println!("  Emoji: {}", emoji_renderer.render_large(&word.emoji));
            println!();
            println!("  Japanese: {}", word.japanese.bright_cyan().bold());
            println!("  Romanji: {}", word.romanji.bright_black());
            if let Some(ref kanji) = word.kanji {
                println!("  Kanji: {}", kanji.bright_cyan());
            }
            println!();
            println!("{}", "Study this word, then press Enter...".bright_blue());
            
            let mut _temp = String::new();
            io::stdin().read_line(&mut _temp).unwrap();
            
            clear_screen();
            toolbar.render();
            println!();
        }
        
        println!("{} {}", "Emoji:".bright_blue().bold(), emoji_renderer.render_large(&word.emoji));
        println!();
        
        let user_input = match read_input_with_shortcuts() {
            Ok(result) => result,
            Err(_) => {
                println!("{}", "Input error!".red());
                continue;
            }
        };

        match user_input {
            InputResult::Quit => {
                clear_screen();
                println!("\n{}", "Saving...".bright_yellow());
                game_state.stats = sr.get_stats().clone();
                game_state.save();
                break;
            }
            InputResult::Menu => {
                show_group_selection_menu(&mut game_state);
                continue;
            }
            InputResult::Retry => {
                println!("{}", "Retrying...".yellow());
                continue;
            }
            InputResult::Skip => {
                println!();
                println!("{}", "Skipped".yellow());
                print!("  Correct: {} ({})", word.japanese.bright_cyan(), word.romanji.bright_black());
                if let Some(ref kanji) = word.kanji {
                    print!(" or {}", kanji.bright_cyan());
                }
                println!();
            }
            InputResult::Answer(answer) => {
                let correct = is_correct_answer(&answer, &word.japanese, &word.romanji, word.kanji.as_deref());
                
                println!();
                if correct {
                    println!("{} ✓", "Correct!".bright_green().bold());
                } else {
                    println!("{} ✗", "Incorrect!".bright_red().bold());
                }
                
                print!("  Correct: {} ({})", word.japanese.bright_cyan(), word.romanji.bright_black());
                if let Some(ref kanji) = word.kanji {
                    print!(" or {}", kanji.bright_cyan());
                }
                println!();

                sr.update_after_answer(word_id, correct);
            }
        }
        
        game_state.stats = sr.get_stats().clone();
        let old_groups = game_state.unlocked_groups.clone();
        game_state.check_and_unlock_next_group(&sr);
        
        if game_state.unlocked_groups.len() > old_groups.len() {
            println!();
            println!("{}", "🎉 New group unlocked! 🎉".bright_yellow().bold());
        }
        
        println!();
        println!("Press Enter...");
        let mut _temp = String::new();
        io::stdin().read_line(&mut _temp).unwrap();
    }

    println!("{}", "がんばって！ (Ganbatte!)".bright_yellow());
}
