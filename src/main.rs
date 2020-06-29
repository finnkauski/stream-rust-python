#![allow(dead_code)]
type FilePath<'a> = &'a str;
type Hz = f32;
type Pulse = f32;
type Notes = Vec<Note>;
type Seconds = f32;
type Bytes = Vec<u8>;
type Semitone = i32;

const PITCH_STANDARD: Hz = 440.0;
const VOLUME: f32 = 0.3;
const SAMPLES: f32 = 48000.0;
const ATTACK: f32 = 0.0004;
const BPM: i32 = 60;

#[derive(Debug)]
struct Note {
    semitones: Semitone,
    duration: Seconds,
}

struct Song {
    notes: Notes,
}

impl Note {
    fn new(semitones: Semitone, which_type: NoteType) -> Self {
        Note {
            semitones,
            duration: which_type.into(),
        }
    }
    fn freq(&self) -> Hz {
        let a = (2.0 as f32).powf(1.0 / 12.0);
        PITCH_STANDARD * a.powi(self.semitones)
    }
}

impl From<Note> for Bytes {
    fn from(note: Note) -> Self {
        let n_entries = (SAMPLES * note.duration) as usize;
        let mut bytes: Vec<u8> = Vec::with_capacity(n_entries);

        let step = (note.freq() * 2.0 * std::f32::consts::PI) / SAMPLES;
        let mut attack = 0.0;
        for n in 0..n_entries {
            if attack < 1.0 {
                attack += ATTACK;
            };
            let converted: [u8; 4] = (attack * VOLUME * (step * n as f32).sin()).to_be_bytes();
            for byte in &converted {
                bytes.push(*byte);
            }
        }
        bytes
    }
}

impl From<Song> for Bytes {
    fn from(song: Song) -> Self {
        song.notes.into_iter().map(Bytes::from).flatten().collect()
    }
}

impl Song {
    fn save(self, filepath: FilePath) -> Result<(), std::io::Error> {
        use std::io::Write;
        let mut file = std::fs::File::create(filepath)?;
        let bytes = Bytes::from(self);
        file.write_all(&bytes[..])?;
        Ok(())
    }
}

pub enum NoteType {
    Quarter,
    Eight,
    Sixteenth,
    Full,
}

impl From<NoteType> for Seconds {
    fn from(notetype: NoteType) -> Self {
        use NoteType::*;
        let frac = match notetype {
            Full => 1.0,
            Quarter => 1.0 / 4.0,
            Eight => 1.0 / 8.0,
            Sixteenth => 1.0 / 16.0,
        };

        let seconds_per_beat = 60.0 / BPM as f32;
        frac * seconds_per_beat
    }
}

fn main() {
    use NoteType::*;
    Song {
        notes: vec![
            Note::new(0, Quarter),
            Note::new(2, Quarter),
            Note::new(4, Eight),
            Note::new(5, Eight),
            Note::new(7, Eight),
            Note::new(9, Sixteenth),
            Note::new(10, Quarter),
            Note::new(11, Quarter),
            Note::new(12, Sixteenth),
        ],
    }
    .save("output.bin")
    .expect("Could not save song!");
}
