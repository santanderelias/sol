# 🎌 Japanese Emoji Learning App

A terminal-based interactive game to learn Japanese vocabulary by matching emojis to their Japanese words. Features spaced repetition for effective learning and progressive difficulty with unlockable word groups.

## ✨ Features

- **🎮 Interactive Learning**: Match emojis to Japanese words in a fun, game-like environment
- **🧠 Spaced Repetition System (SSRS)**: Smart algorithm that shows words you struggle with more frequently
- **📊 Mastery-Based Progression**: Unlock new groups by mastering 80% of current words (5+ correct each)
- **📖 First-Time Learning Mode**: New words show the answer first, then test your knowledge
- **⌨️ Triple Input Support**: Answer in Japanese (ひらがな/カタカナ), Kanji (漢字), or Romanji
- **💾 Progress Tracking**: Your learning progress is automatically saved and restored
- **🎨 Colorful Terminal UI**: Beautiful, easy-to-read interface with color-coded feedback
- **🧹 Clean Display**: Terminal clears between questions for a distraction-free experience
- **🎯 Manual Group Selection**: Choose which word groups to practice from an interactive menu
- **🌈 Colored Emoji Support**: Modern terminals display vibrant, colorful emojis for better visibility
- **⏱️ Keyboard Shortcuts**: Ctrl+Q/R/S/M for quick actions (quit/retry/skip/menu)
- **📊 Persistent Toolbar**: Always-visible stats showing progress and available shortcuts
- **📝 Placement Test**: Advanced users can test out of known content and skip to appropriate level

## 📋 Requirements

- **Rust** (1.70 or later)
- **Cargo** (comes with Rust)

## 🚀 Installation

### 1. Install Rust (if not already installed)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Follow the on-screen instructions and restart your terminal.

### 2. Clone or Download this Repository

```bash
cd /path/to/emoji_japanese_learner
```

### 3. Build the Project

Use the provided build script that checks for dependencies:

```bash
./build.sh
```

The script will:
- ✓ Check if Rust and Cargo are installed
- ✓ Skip installation if dependencies are already met
- ✓ Build the project in release mode for optimal performance

## 🎯 How to Play

### Running the App

```bash
cargo run --release
```

Or run the compiled binary directly:

```bash
./target/release/emoji_japanese_learner
```

### Gameplay

1. **Choose Mode**: Start learning or take placement test
2. **View the Emoji**: An emoji will be displayed on screen (in color on modern terminals!)
3. **Type Your Answer**: Enter the Japanese word in hiragana/katakana, kanji, or romanji
4. **Get Feedback**: Immediate feedback shows if you're correct
5. **Progress**: Answer 5+ times correctly per word to master it and unlock new groups

### Keyboard Shortcuts

Use these shortcuts anytime during gameplay:

- **Ctrl+Q** - Quit and save progress
- **Ctrl+R** - Retry (clear input buffer if IME causes issues)
- **Ctrl+S** - Skip current word (no penalty)
- **Ctrl+M** - Open group selection menu

### Placement Test

For advanced users who already know some Japanese:

1. Choose "Placement Test" at startup
2. Test shows all words from each group in order
3. Get ALL correct → group marked as mastered, move to next
4. Get ANY wrong → test stops, start learning from that group
5. Test is resumable - can exit and continue later

### Example Session

```
🎌 Japanese Emoji Learning Game 🎌
Learn Japanese vocabulary by matching emojis to their words!
You can answer in Japanese (ひらがな/カタカナ), Kanji (漢字), or Romanji

Emoji: 🐱
Your answer: 猫
Correct! ✓
  Correct answers: ねこ (neko) or 猫

Emoji: 🐶
Your answer: inu
Correct! ✓
  Correct answers: いぬ (inu) or 犬
```

## 📚 Word Groups

The app includes **12 themed groups with 84 words total**:

- **Group 1: Animals** 🐱🐶🐦🐟🐘🐻🐰 (7 words)
- **Group 2: Food & Drinks** 🍎🍚🍵🍞🍜🍰🥛 (7 words)
- **Group 3: Nature** 🌸🌊⛰️🌙☀️🌳🌺 (7 words)
- **Group 4: Objects** 📚✏️🚗🏠⌚📱💻 (7 words)
- **Group 5: Colors** 🔴🔵🟡🟢⚫⚪🟠 (7 words)
- **Group 6: Body Parts** 👁️👂👃👄✋🦶💪 (7 words)
- **Group 7: Family** 👨👩👦👧👶👴👵 (7 words)
- **Group 8: Fruits** 🍊🍌🍇🍓🍑🍉🍒 (7 words)
- **Group 9: Vegetables** 🥕🥔🌽🥒🍅🥬🧅 (7 words)
- **Group 10: Weather** ☀️☁️🌧️⛈️❄️🌈⚡ (7 words)
- **Group 11: Numbers** 1️⃣2️⃣3️⃣4️⃣5️⃣6️⃣7️⃣8️⃣9️⃣🔟 (10 words)
- **Group 12: Time** 🌅🌆🌃🕐📅📆🗓️ (7 words)

## 🧠 How the Learning System Works

### Mastery-Based Progression

The app uses a **research-backed mastery system** with four levels:

1. **New** (0 correct) - Word shown with answer first in learning mode
2. **Learning** (1-2 correct) - Still building familiarity
3. **Young** (3-4 correct) - Getting comfortable
4. **Mature** (5+ correct) - Mastered!

**Group Unlock Requirement**: You must reach **Mature** level (5+ correct answers) on **80% of words** in your current groups before unlocking the next group.

**Example**: For a 7-word group, you need 6 words with 5+ correct answers each = approximately 30 total correct answers.

This ensures solid learning before advancing to new material.

### Spaced Repetition Algorithm

The app uses an **SM-2 inspired algorithm** that:

1. **Prioritizes new words**: Words you haven't seen yet appear first
2. **Tracks difficulty**: Words you struggle with get a lower "ease factor"
3. **Adjusts intervals**: Correct answers increase the time before you see the word again
4. **Focuses on weak areas**: Incorrect answers reset the interval, showing the word sooner

This ensures efficient learning by focusing your practice on words that need the most attention.

## 🔧 Troubleshooting

### IME Keyboard Switching Issues

If you experience issues when switching between Japanese IME and English keyboard (e.g., characters get stuck and can't be deleted):

- **Type `retry`**: Clears the input buffer and lets you try again
- **Type `skip`**: Skips the current word without penalty (won't affect your statistics)

These commands help work around terminal input buffer issues that can occur when switching input methods mid-composition.

### Example
```
Your answer: ね[switch to English keyboard, backspace doesn't work]
Your answer: retry
Retrying... (clearing input buffer)

Your answer: neko
Correct! ✓
```

## 💾 Progress Storage

Your progress is automatically saved to:
- **Linux/Mac**: `~/.config/emoji_japanese_learner/progress.json`
- **Windows**: `%APPDATA%\emoji_japanese_learner\progress.json`

The save file includes:
- Unlocked word groups
- Statistics for each word (correct/incorrect counts, ease factor, intervals)
- Last seen timestamps

## 🛠️ Technical Details

### Project Structure

```
emoji_japanese_learner/
├── src/
│   ├── main.rs              # Main game loop and UI
│   ├── word_data.rs         # Word definitions and groups
│   ├── spaced_repetition.rs # SSRS algorithm implementation
│   ├── input_validator.rs   # Japanese/Romanji input validation
│   └── game_state.rs        # Save/load and progression logic
├── Cargo.toml               # Dependencies
├── build.sh                 # Build script with dependency checks
└── README.md                # This file
```

### Dependencies

- `serde` & `serde_json` - Serialization for save files
- `colored` - Terminal color output
- `unicode-normalization` - Japanese text normalization
- `dirs` - Cross-platform config directory

## 🎓 Learning Tips

1. **Be Consistent**: Practice a little bit every day for best results
2. **Use Both Input Methods**: Try typing in Japanese to reinforce character recognition
3. **Don't Rush**: The spaced repetition will naturally adjust to your pace
4. **Review Mistakes**: Pay attention to the correct answers when you get them wrong

## 📝 License

This project is open source and available for educational purposes.

## 🤝 Contributing

Feel free to add more word groups, improve the algorithm, or enhance the UI!

---

**がんばって！ (Ganbatte!)** - Good luck with your Japanese learning journey! 🎌
