use parking_lot::Mutex;
use std::sync::Arc;

use crate::settings::{
    EFFECT_FADEOUT, EFFECT_NONE, EFFECT_SLIDE, EFFECT_VIBRATO, INITIAL_SPEED,
    RESOURCE_ARCHIVE_DIRNAME, TONE_NOISE, TONE_PULSE, TONE_SQUARE, TONE_TRIANGLE,
};
use crate::types::{Effect, Note, Speed, Tone, Volume};
use crate::utils::simplify_string;

#[derive(Clone)]
pub struct Sound {
    pub notes: Vec<Note>,
    pub tones: Vec<Tone>,
    pub volumes: Vec<Volume>,
    pub effects: Vec<Effect>,
    pub speed: Speed,
}

pub type SharedSound = Arc<Mutex<Sound>>;

impl Sound {
    pub fn new() -> SharedSound {
        Arc::new(Mutex::new(Sound {
            notes: Vec::new(),
            tones: Vec::new(),
            volumes: Vec::new(),
            effects: Vec::new(),
            speed: INITIAL_SPEED,
        }))
    }

    pub fn set(
        &mut self,
        note_str: &str,
        tone_str: &str,
        volume_str: &str,
        effect_str: &str,
        speed: Speed,
    ) {
        self.set_note(note_str);
        self.set_tone(tone_str);
        self.set_volume(volume_str);
        self.set_effect(effect_str);
        self.speed = speed;
    }

    pub fn set_note(&mut self, note_str: &str) {
        let note_str = simplify_string(note_str);
        let mut chars = note_str.chars();

        self.notes.clear();

        while let Some(c) = chars.next() {
            let mut note: Note;

            if c >= 'a' && c <= 'g' {
                note = match c {
                    'c' => 0,
                    'd' => 2,
                    'e' => 4,
                    'f' => 5,
                    'g' => 7,
                    'a' => 9,
                    'b' => 11,
                    _ => panic!("invalid sound note '{}'", c),
                };
                let mut c = chars.next().unwrap_or(0 as char);

                if c == '#' {
                    note += 1;
                    c = chars.next().unwrap_or(0 as char);
                } else if c == '-' {
                    note -= 1;
                    c = chars.next().unwrap_or(0 as char);
                }

                if c >= '0' && c <= '4' {
                    note += (c as Note - '0' as Note) * 12;
                } else {
                    panic!("invalid sound note '{}'", c);
                }
            } else if c == 'r' {
                note = -1;
            } else {
                panic!("invalid sound note '{}'", c);
            }

            self.notes.push(note);
        }
    }

    pub fn set_tone(&mut self, tone_str: &str) {
        let tone_str = simplify_string(tone_str);
        let mut chars = tone_str.chars();

        self.tones.clear();

        while let Some(c) = chars.next() {
            let tone = match c {
                't' => TONE_TRIANGLE,
                's' => TONE_SQUARE,
                'p' => TONE_PULSE,
                'n' => TONE_NOISE,
                _ => panic!("invalid sound tone '{}'", c),
            };

            self.tones.push(tone);
        }
    }

    pub fn set_volume(&mut self, volume_str: &str) {
        let volume_str = simplify_string(volume_str);
        let mut chars = volume_str.chars();

        self.volumes.clear();

        while let Some(c) = chars.next() {
            if c >= '0' && c <= '7' {
                self.volumes.push((c as u32 - '0' as u32) as Volume);
            } else {
                panic!("invalid sound volume '{}'", c);
            }
        }
    }

    pub fn set_effect(&mut self, effect_str: &str) {
        let effect_str = simplify_string(effect_str);
        let mut chars = effect_str.chars();

        self.effects.clear();

        while let Some(c) = chars.next() {
            let effect = match c {
                'n' => EFFECT_NONE,
                's' => EFFECT_SLIDE,
                'v' => EFFECT_VIBRATO,
                'f' => EFFECT_FADEOUT,
                _ => panic!("invalid sound effect '{}'", c),
            };

            self.effects.push(effect);
        }
    }

    pub(crate) fn resource_name(sound_no: u32) -> String {
        RESOURCE_ARCHIVE_DIRNAME.to_string() + "sound" + &format!("{:02}", sound_no)
    }

    pub fn clear(&mut self) {
        //
    }

    pub(crate) fn serialize(&self) -> String {
        /*
        Sound* sound = audio_->GetSoundBank(sound_index);

        if (sound->Note().size() == 0 && sound->Tone().size() == 0 &&
            sound->Volume().size() == 0 && sound->Effect().size() == 0) {
          return "";
        }

        std::stringstream ss;

        ss << std::hex;

        if (sound->Note().size() > 0) {
          for (int32_t v : sound->Note()) {
            if (v < 0) {
              v = 0xff;
            }

            ss << std::setw(2) << std::setfill('0') << v;
          }
          ss << std::endl;
        } else {
          ss << "none" << std::endl;
        }

        if (sound->Tone().size() > 0) {
          for (int32_t v : sound->Tone()) {
            ss << v;
          }
          ss << std::endl;
        } else {
          ss << "none" << std::endl;
        }

        if (sound->Volume().size() > 0) {
          for (int32_t v : sound->Volume()) {
            ss << v;
          }
          ss << std::endl;
        } else {
          ss << "none" << std::endl;
        }

        if (sound->Effect().size() > 0) {
          for (int32_t v : sound->Effect()) {
            ss << v;
          }
          ss << std::endl;
        } else {
          ss << "none" << std::endl;
        }

        ss << std::dec << sound->Speed() << std::endl;

        return ss.str();
        */
        "TODO".to_string()
    }

    pub(crate) fn deserialize(&mut self, input: &str) {
        /*
        Sound* sound = audio_->GetSoundBank(sound_index);
        std::stringstream ss(str);

        {
        std::string line;
        std::getline(ss, line);
        line = Trim(line);

        SoundData& note = sound->Note();
        note.clear();

        if (line != "none") {
            for (int32_t i = 0; i < line.size() / 2; i++) {
            int32_t v = std::stoi(line.substr(i * 2, 2), nullptr, 16);

            if (v == 0xff) {
                v = -1;
            }

            note.push_back(v);
            }
        }

        PARSE_SOUND(ss, sound, Tone);
        PARSE_SOUND(ss, sound, Volume);
        PARSE_SOUND(ss, sound, Effect);

        {
            std::string line;
            std::getline(ss, line);
            line = Trim(line);

            sound->Speed(std::stoi(line));
        }

        #define PARSE_SOUND(ss, sound, property)   \
          do {                                     \
            SoundData& data = sound->property();   \
            data.clear();                          \
                                                   \
            std::string line = GetTrimmedLine(ss); \
                                                   \
            if (line != "none") {                  \
              for (char c : line) {                \
                data.push_back(c - '0');           \
              }                                    \
            }                                      \
          } while (false)

        */
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let sound = Sound::new();
        assert_eq!(sound.lock().notes.len(), 0);
        assert_eq!(sound.lock().tones.len(), 0);
        assert_eq!(sound.lock().volumes.len(), 0);
        assert_eq!(sound.lock().effects.len(), 0);
        assert_eq!(sound.lock().speed, INITIAL_SPEED);
    }

    #[test]
    fn set() {
        let sound = Sound::new();

        sound.lock().set("c0d-0d0d#0", "tspn", "0123", "nsvf", 123);
        assert_eq!(&sound.lock().notes, &vec![0, 1, 2, 3]);
        assert_eq!(
            &sound.lock().tones,
            &vec![TONE_TRIANGLE, TONE_SQUARE, TONE_PULSE, TONE_NOISE]
        );
        assert_eq!(&sound.lock().volumes, &vec![0, 1, 2, 3]);
        assert_eq!(
            &sound.lock().effects,
            &vec![EFFECT_NONE, EFFECT_SLIDE, EFFECT_VIBRATO, EFFECT_FADEOUT]
        );
        assert_eq!(sound.lock().speed, 123);
    }

    #[test]
    fn set_note() {
        let sound = Sound::new();

        sound
            .lock()
            .set_note(" c 0 d # 1 r e 2 f 3 g 4 r a - 0 b 1 ");
        assert_eq!(&sound.lock().notes, &vec![0, 15, -1, 28, 41, 55, -1, 8, 23]);
    }

    #[test]
    fn set_tone() {
        let sound = Sound::new();

        sound.lock().set_tone(" t s p n ");
        assert_eq!(
            &sound.lock().tones,
            &vec![TONE_TRIANGLE, TONE_SQUARE, TONE_PULSE, TONE_NOISE]
        );
    }

    #[test]
    fn set_volume() {
        let sound = Sound::new();

        sound.lock().set_volume(" 0 1 2 3 4 5 6 7 ");
        assert_eq!(&sound.lock().volumes, &vec![0, 1, 2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn set_effect() {
        let sound = Sound::new();

        sound.lock().set_effect(" n s v f ");
        assert_eq!(
            &sound.lock().effects,
            &vec![EFFECT_NONE, EFFECT_SLIDE, EFFECT_VIBRATO, EFFECT_FADEOUT]
        );
    }
}
