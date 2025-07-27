use crate::journal::emotion::map_emotion_to_color;

#[derive(Debug, Clone)]
pub struct EmotionEntry {
    pub date: String,
    pub emotion: String,
    pub color: String,
    pub note: Option<String>,
}

impl EmotionEntry {
    pub fn new(date: String, emotion: String, note: Option<String>) -> Self {
        let color = map_emotion_to_color(&emotion);
        EmotionEntry { date, emotion, color, note }
    }
}
