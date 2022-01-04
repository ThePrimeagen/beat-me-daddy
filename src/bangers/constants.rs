pub const BIT_LENGTH: usize = 10;
pub const STARTING_UTF: char = '♥';
pub const STARTING_UTF_OFFSET: u32 = '♥' as u32;

// TODO: BEAT_COUNT really needs to be driven by the server
// We need a source of truth
pub const BEAT_COUNT: usize = 64;

pub const DRUM_NAMES: [&str; 22] = [
    "bd_pure",
    "bd_boom",
    "drum_cowbell",
    "drum_roll",
    "drum_heavy_kick",
    "drum_tom_mid_soft",
    "drum_tom_mid_hard",
    "drum_tom_lo_soft",
    "drum_tom_lo_hard",
    "drum_tom_hi_soft",
    "drum_tom_hi_hard",
    "drum_splash_soft",
    "drum_splash_hard",
    "drum_snare_soft",
    "drum_snare_hard",
    "drum_cymbal_soft",
    "drum_cymbal_hard",
    "drum_cymbal_open",
    "drum_cymbal_closed",
    "drum_cymbal_pedal",
    "drum_bass_soft",
    "drum_bass_hard",
];
