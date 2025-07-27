pub fn map_emotion_to_color(emotion: &str) -> String {
    match emotion.to_lowercase().as_str() {
        "happy" => "Yellow".to_string(),
        "sad" => "Blue".to_string(),
        "angry" => "Red".to_string(),
        "anxious" => "Purple".to_string(),
        "excited" => "Orange".to_string(),
        "relaxed" => "Green".to_string(),
        "grateful" => "Pink".to_string(),
        _ => "Gray".to_string(),
    }
}
