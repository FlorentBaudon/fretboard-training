use rand::Rng;

/// Natural notes (unmodified) - English format
const NATURAL_NOTES: &[&str] = &["C", "D", "E", "F", "G", "A", "B"];

/// Sharp notes - English format
const SHARP_NOTES: &[&str] = &["C#", "D#", "F#", "G#", "A#"];

/// Flat notes - English format
const FLAT_NOTES: &[&str] = &["Db", "Eb", "Gb", "Ab", "Bb"];

/// Converts a note from English format to classical format (solfege)
pub fn to_solfege(note: &str) -> String {
    match note {
        "C" => "Do".to_string(),
        "D" => "Ré".to_string(),
        "E" => "Mi".to_string(),
        "F" => "Fa".to_string(),
        "G" => "Sol".to_string(),
        "A" => "La".to_string(),
        "B" => "Si".to_string(),
        "C#" => "Do#".to_string(),
        "D#" => "Ré#".to_string(),
        "F#" => "Fa#".to_string(),
        "G#" => "Sol#".to_string(),
        "A#" => "La#".to_string(),
        "Db" => "Ré♭".to_string(),
        "Eb" => "Mi♭".to_string(),
        "Gb" => "Sol♭".to_string(),
        "Ab" => "La♭".to_string(),
        "Bb" => "Si♭".to_string(),
        _ => note.to_string(), // If the note is not recognized, return it as is
    }
}

/// Converts a note from classical format (solfege) to English format
pub fn from_solfege(note: &str) -> String {
    match note {
        "Do" => "C".to_string(),
        "Ré" => "D".to_string(),
        "Mi" => "E".to_string(),
        "Fa" => "F".to_string(),
        "Sol" => "G".to_string(),
        "La" => "A".to_string(),
        "Si" => "B".to_string(),
        "Do#" => "C#".to_string(),
        "Ré#" => "D#".to_string(),
        "Fa#" => "F#".to_string(),
        "Sol#" => "G#".to_string(),
        "La#" => "A#".to_string(),
        "Ré♭" | "Réb" => "Db".to_string(),
        "Mi♭" | "Mib" => "Eb".to_string(),
        "Sol♭" | "Solb" => "Gb".to_string(),
        "La♭" | "Lab" => "Ab".to_string(),
        "Si♭" | "Sib" => "Bb".to_string(),
        _ => note.to_string(), // If the note is not recognized, return it as is
    }
}

/// Generates a random note
/// 
/// # Arguments
/// * `allow_sharp` - If true, includes modified notes (sharps and flats)
///                   If false, generates only natural notes
/// * `use_solfege` - If true, returns the note in classical format (Do, Ré, Mi...)
///                   If false, returns the note in English format (C, D, E...)
pub fn generate_random_note(allow_sharp: bool, use_solfege: bool) -> String {
    let mut rng = rand::thread_rng();
    
    let note = if allow_sharp {
        // Mode with modifications: can have natural, sharp, or flat notes
        let all_notes: Vec<&str> = [NATURAL_NOTES, SHARP_NOTES, FLAT_NOTES]
            .iter()
            .flat_map(|notes| notes.iter().copied())
            .collect();
        
        let index = rng.gen_range(0..all_notes.len());
        all_notes[index].to_string()
    } else {
        // Mode without modifications: only natural notes
        let index = rng.gen_range(0..NATURAL_NOTES.len());
        NATURAL_NOTES[index].to_string()
    };
    
    if use_solfege {
        to_solfege(&note)
    } else {
        note
    }
}

/// Converts an existing note between English and classical formats
pub fn convert_note_format(note: &str, to_solfege_format: bool) -> String {
    if to_solfege_format {
        to_solfege(note)
    } else {
        from_solfege(note)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_natural_note() {
        let note = generate_random_note(false, false);
        assert!(NATURAL_NOTES.contains(&note.as_str()));
    }

    #[test]
    fn test_generate_note_with_sharp() {
        let note = generate_random_note(true, false);
        let all_notes: Vec<&str> = [NATURAL_NOTES, SHARP_NOTES, FLAT_NOTES]
            .iter()
            .flat_map(|notes| notes.iter().copied())
            .collect();
        assert!(all_notes.contains(&note.as_str()));
    }

    #[test]
    fn test_to_solfege() {
        assert_eq!(to_solfege("C"), "Do");
        assert_eq!(to_solfege("C#"), "Do#");
        assert_eq!(to_solfege("Db"), "Ré♭");
    }

    #[test]
    fn test_from_solfege() {
        assert_eq!(from_solfege("Do"), "C");
        assert_eq!(from_solfege("Do#"), "C#");
        assert_eq!(from_solfege("Ré♭"), "Db");
    }
}

