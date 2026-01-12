use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Word {
    pub emoji: String,
    pub japanese: String,
    pub romanji: String,
    pub kanji: Option<String>,
    pub group: usize,
}

#[derive(Debug, Clone)]
pub struct GroupInfo {
    pub id: usize,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MasteryLevel {
    New,        // 0 correct
    Learning,   // 1-2 correct
    Young,      // 3-4 correct
    Mature,     // 5+ correct
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WordStats {
    pub word_id: usize,
    pub correct_count: u32,
    pub incorrect_count: u32,
    pub last_seen: Option<u64>,
    pub ease_factor: f32,
    pub interval: u32,
}

impl WordStats {
    pub fn new(word_id: usize) -> Self {
        WordStats {
            word_id,
            correct_count: 0,
            incorrect_count: 0,
            last_seen: None,
            ease_factor: 2.5,
            interval: 0,
        }
    }

    pub fn get_mastery_level(&self) -> MasteryLevel {
        match self.correct_count {
            0 => MasteryLevel::New,
            1..=2 => MasteryLevel::Learning,
            3..=4 => MasteryLevel::Young,
            _ => MasteryLevel::Mature,
        }
    }

    pub fn is_new(&self) -> bool {
        self.last_seen.is_none()
    }
}

pub fn get_group_info() -> Vec<GroupInfo> {
    vec![
        GroupInfo { id: 1, name: "Animals".to_string(), description: "Common animals".to_string() },
        GroupInfo { id: 2, name: "Food & Drinks".to_string(), description: "Everyday food and beverages".to_string() },
        GroupInfo { id: 3, name: "Nature".to_string(), description: "Natural elements".to_string() },
        GroupInfo { id: 4, name: "Objects".to_string(), description: "Everyday items".to_string() },
        GroupInfo { id: 5, name: "Colors".to_string(), description: "Basic colors".to_string() },
        GroupInfo { id: 6, name: "Body Parts".to_string(), description: "Parts of the body".to_string() },
        GroupInfo { id: 7, name: "Family".to_string(), description: "Family members".to_string() },
        GroupInfo { id: 8, name: "Fruits".to_string(), description: "Various fruits".to_string() },
        GroupInfo { id: 9, name: "Vegetables".to_string(), description: "Common vegetables".to_string() },
        GroupInfo { id: 10, name: "Weather".to_string(), description: "Weather conditions".to_string() },
        GroupInfo { id: 11, name: "Numbers".to_string(), description: "Numbers 1-10".to_string() },
        GroupInfo { id: 12, name: "Time".to_string(), description: "Time-related words".to_string() },
    ]
}

pub fn get_all_words() -> Vec<Word> {
    vec![
        // Group 1: Animals
        Word { emoji: "🐱".to_string(), japanese: "ねこ".to_string(), romanji: "neko".to_string(), kanji: Some("猫".to_string()), group: 1 },
        Word { emoji: "🐶".to_string(), japanese: "いぬ".to_string(), romanji: "inu".to_string(), kanji: Some("犬".to_string()), group: 1 },
        Word { emoji: "🐦".to_string(), japanese: "とり".to_string(), romanji: "tori".to_string(), kanji: Some("鳥".to_string()), group: 1 },
        Word { emoji: "🐟".to_string(), japanese: "さかな".to_string(), romanji: "sakana".to_string(), kanji: Some("魚".to_string()), group: 1 },
        Word { emoji: "🐘".to_string(), japanese: "ぞう".to_string(), romanji: "zou".to_string(), kanji: Some("象".to_string()), group: 1 },
        Word { emoji: "🐻".to_string(), japanese: "くま".to_string(), romanji: "kuma".to_string(), kanji: Some("熊".to_string()), group: 1 },
        Word { emoji: "🐰".to_string(), japanese: "うさぎ".to_string(), romanji: "usagi".to_string(), kanji: Some("兎".to_string()), group: 1 },
        
        // Group 2: Food & Drinks
        Word { emoji: "🍎".to_string(), japanese: "りんご".to_string(), romanji: "ringo".to_string(), kanji: None, group: 2 },
        Word { emoji: "🍚".to_string(), japanese: "ごはん".to_string(), romanji: "gohan".to_string(), kanji: Some("御飯".to_string()), group: 2 },
        Word { emoji: "🍵".to_string(), japanese: "おちゃ".to_string(), romanji: "ocha".to_string(), kanji: Some("お茶".to_string()), group: 2 },
        Word { emoji: "🍞".to_string(), japanese: "パン".to_string(), romanji: "pan".to_string(), kanji: None, group: 2 },
        Word { emoji: "🍜".to_string(), japanese: "ラーメン".to_string(), romanji: "raamen".to_string(), kanji: None, group: 2 },
        Word { emoji: "🍰".to_string(), japanese: "ケーキ".to_string(), romanji: "keeki".to_string(), kanji: None, group: 2 },
        Word { emoji: "🥛".to_string(), japanese: "ミルク".to_string(), romanji: "miruku".to_string(), kanji: None, group: 2 },
        
        // Group 3: Nature
        Word { emoji: "🌸".to_string(), japanese: "さくら".to_string(), romanji: "sakura".to_string(), kanji: Some("桜".to_string()), group: 3 },
        Word { emoji: "🌊".to_string(), japanese: "うみ".to_string(), romanji: "umi".to_string(), kanji: Some("海".to_string()), group: 3 },
        Word { emoji: "⛰️".to_string(), japanese: "やま".to_string(), romanji: "yama".to_string(), kanji: Some("山".to_string()), group: 3 },
        Word { emoji: "🌙".to_string(), japanese: "つき".to_string(), romanji: "tsuki".to_string(), kanji: Some("月".to_string()), group: 3 },
        Word { emoji: "☀️".to_string(), japanese: "たいよう".to_string(), romanji: "taiyou".to_string(), kanji: Some("太陽".to_string()), group: 3 },
        Word { emoji: "🌳".to_string(), japanese: "き".to_string(), romanji: "ki".to_string(), kanji: Some("木".to_string()), group: 3 },
        Word { emoji: "🌺".to_string(), japanese: "はな".to_string(), romanji: "hana".to_string(), kanji: Some("花".to_string()), group: 3 },
        
        // Group 4: Objects
        Word { emoji: "📚".to_string(), japanese: "ほん".to_string(), romanji: "hon".to_string(), kanji: Some("本".to_string()), group: 4 },
        Word { emoji: "✏️".to_string(), japanese: "えんぴつ".to_string(), romanji: "enpitsu".to_string(), kanji: Some("鉛筆".to_string()), group: 4 },
        Word { emoji: "🚗".to_string(), japanese: "くるま".to_string(), romanji: "kuruma".to_string(), kanji: Some("車".to_string()), group: 4 },
        Word { emoji: "🏠".to_string(), japanese: "いえ".to_string(), romanji: "ie".to_string(), kanji: Some("家".to_string()), group: 4 },
        Word { emoji: "⌚".to_string(), japanese: "とけい".to_string(), romanji: "tokei".to_string(), kanji: Some("時計".to_string()), group: 4 },
        Word { emoji: "📱".to_string(), japanese: "でんわ".to_string(), romanji: "denwa".to_string(), kanji: Some("電話".to_string()), group: 4 },
        Word { emoji: "💻".to_string(), japanese: "パソコン".to_string(), romanji: "pasokon".to_string(), kanji: None, group: 4 },
        
        // Group 5: Colors
        Word { emoji: "🔴".to_string(), japanese: "あか".to_string(), romanji: "aka".to_string(), kanji: Some("赤".to_string()), group: 5 },
        Word { emoji: "🔵".to_string(), japanese: "あお".to_string(), romanji: "ao".to_string(), kanji: Some("青".to_string()), group: 5 },
        Word { emoji: "🟡".to_string(), japanese: "きいろ".to_string(), romanji: "kiiro".to_string(), kanji: Some("黄色".to_string()), group: 5 },
        Word { emoji: "🟢".to_string(), japanese: "みどり".to_string(), romanji: "midori".to_string(), kanji: Some("緑".to_string()), group: 5 },
        Word { emoji: "⚫".to_string(), japanese: "くろ".to_string(), romanji: "kuro".to_string(), kanji: Some("黒".to_string()), group: 5 },
        Word { emoji: "⚪".to_string(), japanese: "しろ".to_string(), romanji: "shiro".to_string(), kanji: Some("白".to_string()), group: 5 },
        Word { emoji: "🟠".to_string(), japanese: "オレンジ".to_string(), romanji: "orenji".to_string(), kanji: None, group: 5 },
        
        // Group 6: Body Parts
        Word { emoji: "👁️".to_string(), japanese: "め".to_string(), romanji: "me".to_string(), kanji: Some("目".to_string()), group: 6 },
        Word { emoji: "👂".to_string(), japanese: "みみ".to_string(), romanji: "mimi".to_string(), kanji: Some("耳".to_string()), group: 6 },
        Word { emoji: "👃".to_string(), japanese: "はな".to_string(), romanji: "hana".to_string(), kanji: Some("鼻".to_string()), group: 6 },
        Word { emoji: "👄".to_string(), japanese: "くち".to_string(), romanji: "kuchi".to_string(), kanji: Some("口".to_string()), group: 6 },
        Word { emoji: "✋".to_string(), japanese: "て".to_string(), romanji: "te".to_string(), kanji: Some("手".to_string()), group: 6 },
        Word { emoji: "🦶".to_string(), japanese: "あし".to_string(), romanji: "ashi".to_string(), kanji: Some("足".to_string()), group: 6 },
        Word { emoji: "💪".to_string(), japanese: "うで".to_string(), romanji: "ude".to_string(), kanji: Some("腕".to_string()), group: 6 },
        
        // Group 7: Family
        Word { emoji: "👨".to_string(), japanese: "ちち".to_string(), romanji: "chichi".to_string(), kanji: Some("父".to_string()), group: 7 },
        Word { emoji: "👩".to_string(), japanese: "はは".to_string(), romanji: "haha".to_string(), kanji: Some("母".to_string()), group: 7 },
        Word { emoji: "👦".to_string(), japanese: "むすこ".to_string(), romanji: "musuko".to_string(), kanji: Some("息子".to_string()), group: 7 },
        Word { emoji: "👧".to_string(), japanese: "むすめ".to_string(), romanji: "musume".to_string(), kanji: Some("娘".to_string()), group: 7 },
        Word { emoji: "👶".to_string(), japanese: "あかちゃん".to_string(), romanji: "akachan".to_string(), kanji: Some("赤ちゃん".to_string()), group: 7 },
        Word { emoji: "👴".to_string(), japanese: "おじいさん".to_string(), romanji: "ojiisan".to_string(), kanji: Some("お祖父さん".to_string()), group: 7 },
        Word { emoji: "👵".to_string(), japanese: "おばあさん".to_string(), romanji: "obaasan".to_string(), kanji: Some("お祖母さん".to_string()), group: 7 },
        
        // Group 8: Fruits
        Word { emoji: "🍊".to_string(), japanese: "みかん".to_string(), romanji: "mikan".to_string(), kanji: Some("蜜柑".to_string()), group: 8 },
        Word { emoji: "🍌".to_string(), japanese: "バナナ".to_string(), romanji: "banana".to_string(), kanji: None, group: 8 },
        Word { emoji: "🍇".to_string(), japanese: "ぶどう".to_string(), romanji: "budou".to_string(), kanji: Some("葡萄".to_string()), group: 8 },
        Word { emoji: "🍓".to_string(), japanese: "いちご".to_string(), romanji: "ichigo".to_string(), kanji: Some("苺".to_string()), group: 8 },
        Word { emoji: "🍑".to_string(), japanese: "もも".to_string(), romanji: "momo".to_string(), kanji: Some("桃".to_string()), group: 8 },
        Word { emoji: "🍉".to_string(), japanese: "すいか".to_string(), romanji: "suika".to_string(), kanji: Some("西瓜".to_string()), group: 8 },
        Word { emoji: "🍒".to_string(), japanese: "さくらんぼ".to_string(), romanji: "sakuranbo".to_string(), kanji: None, group: 8 },
        
        // Group 9: Vegetables
        Word { emoji: "🥕".to_string(), japanese: "にんじん".to_string(), romanji: "ninjin".to_string(), kanji: Some("人参".to_string()), group: 9 },
        Word { emoji: "🥔".to_string(), japanese: "じゃがいも".to_string(), romanji: "jagaimo".to_string(), kanji: None, group: 9 },
        Word { emoji: "🌽".to_string(), japanese: "とうもろこし".to_string(), romanji: "toumorokoshi".to_string(), kanji: None, group: 9 },
        Word { emoji: "🥒".to_string(), japanese: "きゅうり".to_string(), romanji: "kyuuri".to_string(), kanji: Some("胡瓜".to_string()), group: 9 },
        Word { emoji: "🍅".to_string(), japanese: "トマト".to_string(), romanji: "tomato".to_string(), kanji: None, group: 9 },
        Word { emoji: "🥬".to_string(), japanese: "キャベツ".to_string(), romanji: "kyabetsu".to_string(), kanji: None, group: 9 },
        Word { emoji: "🧅".to_string(), japanese: "たまねぎ".to_string(), romanji: "tamanegi".to_string(), kanji: Some("玉葱".to_string()), group: 9 },
        
        // Group 10: Weather
        Word { emoji: "☀️".to_string(), japanese: "はれ".to_string(), romanji: "hare".to_string(), kanji: Some("晴れ".to_string()), group: 10 },
        Word { emoji: "☁️".to_string(), japanese: "くもり".to_string(), romanji: "kumori".to_string(), kanji: Some("曇り".to_string()), group: 10 },
        Word { emoji: "🌧️".to_string(), japanese: "あめ".to_string(), romanji: "ame".to_string(), kanji: Some("雨".to_string()), group: 10 },
        Word { emoji: "⛈️".to_string(), japanese: "あらし".to_string(), romanji: "arashi".to_string(), kanji: Some("嵐".to_string()), group: 10 },
        Word { emoji: "❄️".to_string(), japanese: "ゆき".to_string(), romanji: "yuki".to_string(), kanji: Some("雪".to_string()), group: 10 },
        Word { emoji: "🌈".to_string(), japanese: "にじ".to_string(), romanji: "niji".to_string(), kanji: Some("虹".to_string()), group: 10 },
        Word { emoji: "⚡".to_string(), japanese: "かみなり".to_string(), romanji: "kaminari".to_string(), kanji: Some("雷".to_string()), group: 10 },
        
        // Group 11: Numbers
        Word { emoji: "1️⃣".to_string(), japanese: "いち".to_string(), romanji: "ichi".to_string(), kanji: Some("一".to_string()), group: 11 },
        Word { emoji: "2️⃣".to_string(), japanese: "に".to_string(), romanji: "ni".to_string(), kanji: Some("二".to_string()), group: 11 },
        Word { emoji: "3️⃣".to_string(), japanese: "さん".to_string(), romanji: "san".to_string(), kanji: Some("三".to_string()), group: 11 },
        Word { emoji: "4️⃣".to_string(), japanese: "よん".to_string(), romanji: "yon".to_string(), kanji: Some("四".to_string()), group: 11 },
        Word { emoji: "5️⃣".to_string(), japanese: "ご".to_string(), romanji: "go".to_string(), kanji: Some("五".to_string()), group: 11 },
        Word { emoji: "6️⃣".to_string(), japanese: "ろく".to_string(), romanji: "roku".to_string(), kanji: Some("六".to_string()), group: 11 },
        Word { emoji: "7️⃣".to_string(), japanese: "なな".to_string(), romanji: "nana".to_string(), kanji: Some("七".to_string()), group: 11 },
        Word { emoji: "8️⃣".to_string(), japanese: "はち".to_string(), romanji: "hachi".to_string(), kanji: Some("八".to_string()), group: 11 },
        Word { emoji: "9️⃣".to_string(), japanese: "きゅう".to_string(), romanji: "kyuu".to_string(), kanji: Some("九".to_string()), group: 11 },
        Word { emoji: "🔟".to_string(), japanese: "じゅう".to_string(), romanji: "juu".to_string(), kanji: Some("十".to_string()), group: 11 },
        
        // Group 12: Time
        Word { emoji: "🌅".to_string(), japanese: "あさ".to_string(), romanji: "asa".to_string(), kanji: Some("朝".to_string()), group: 12 },
        Word { emoji: "🌆".to_string(), japanese: "ゆうがた".to_string(), romanji: "yuugata".to_string(), kanji: Some("夕方".to_string()), group: 12 },
        Word { emoji: "🌃".to_string(), japanese: "よる".to_string(), romanji: "yoru".to_string(), kanji: Some("夜".to_string()), group: 12 },
        Word { emoji: "🕐".to_string(), japanese: "いちじ".to_string(), romanji: "ichiji".to_string(), kanji: Some("一時".to_string()), group: 12 },
        Word { emoji: "📅".to_string(), japanese: "ひ".to_string(), romanji: "hi".to_string(), kanji: Some("日".to_string()), group: 12 },
        Word { emoji: "📆".to_string(), japanese: "しゅう".to_string(), romanji: "shuu".to_string(), kanji: Some("週".to_string()), group: 12 },
        Word { emoji: "🗓️".to_string(), japanese: "つき".to_string(), romanji: "tsuki".to_string(), kanji: Some("月".to_string()), group: 12 },
    ]
}
