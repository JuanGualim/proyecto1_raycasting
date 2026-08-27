use std::{f32::consts::TAU, sync::OnceLock};

use raylib::prelude::{Music, RaylibAudio};

const SAMPLE_RATE: u32 = 22_050;
const MUSIC_DURATION_SECONDS: u32 = 16;
const CHANNELS: u16 = 1;
const BITS_PER_SAMPLE: u16 = 16;

static BACKGROUND_MUSIC_WAV: OnceLock<Vec<u8>> = OnceLock::new();

pub struct AudioSystem<'audio> {
    music: Music<'audio>,
    enabled: bool,
}

impl<'audio> AudioSystem<'audio> {
    pub fn new(device: &'audio RaylibAudio) -> Result<Self, String> {
        let bytes = BACKGROUND_MUSIC_WAV.get_or_init(compose_background_music_wav);
        let mut music = device
            .new_music_from_memory(".wav", bytes)
            .map_err(|error| format!("no se pudo crear la musica: {error}"))?;
        music.set_looping(true);
        music.set_volume(crate::config::MUSIC_VOLUME);
        music.play_stream();

        Ok(Self {
            music,
            enabled: true,
        })
    }

    pub fn update(&mut self, enabled: bool) {
        if self.enabled != enabled {
            self.enabled = enabled;
            self.music.set_volume(if enabled {
                crate::config::MUSIC_VOLUME
            } else {
                0.0
            });
        }

        self.music.update_stream();
        if !self.music.is_stream_playing() {
            self.music.play_stream();
        }
    }
}

fn compose_background_music_wav() -> Vec<u8> {
    encode_wav(&compose_background_music_samples(), SAMPLE_RATE)
}

fn compose_background_music_samples() -> Vec<i16> {
    let frame_count = (SAMPLE_RATE * MUSIC_DURATION_SECONDS) as usize;
    let duration = MUSIC_DURATION_SECONDS as f32;
    let roots = [146.83_f32, 116.54, 174.61, 130.81];
    let arpeggio = [1.0_f32, 1.2, 1.5, 2.0, 1.5, 1.2, 1.8, 1.5];
    let mut samples = Vec::with_capacity(frame_count);

    for frame in 0..frame_count {
        let time = frame as f32 / SAMPLE_RATE as f32;
        let section = ((time / 4.0).floor() as usize) % roots.len();
        let section_time = time % 4.0;
        let next_section = (section + 1) % roots.len();
        let blend = smooth_step(((section_time - 3.4) / 0.6).clamp(0.0, 1.0));

        let current_pad = temple_pad(roots[section], time);
        let next_pad = temple_pad(roots[next_section], time);
        let pad = current_pad * (1.0 - blend) + next_pad * blend;

        let note_length = 0.5;
        let note_index = ((time / note_length).floor() as usize) % arpeggio.len();
        let note_progress = (time % note_length) / note_length;
        let note_envelope = (note_progress * std::f32::consts::PI).sin().powi(2);
        let note_frequency = roots[section] * arpeggio[note_index] * 2.0;
        let arpeggio_voice = (TAU * note_frequency * time).sin() * note_envelope * 0.24;

        let bell_envelope = (-2.4 * section_time).exp();
        let bell = ((TAU * roots[section] * 4.0 * time).sin()
            + 0.35 * (TAU * roots[section] * 6.03 * time).sin())
            * bell_envelope
            * 0.18;

        let pulse_time = time % 2.0;
        let pulse = (TAU * 52.0 * time).sin() * (-18.0 * pulse_time).exp() * 0.12;
        let shimmer = ((TAU * 987.77 * time).sin() * (TAU * 0.125 * time).sin()) * 0.025;
        let loop_fade = (time / 0.08).min((duration - time) / 0.08).clamp(0.0, 1.0);
        let value = (pad + arpeggio_voice + bell + pulse + shimmer) * loop_fade * 0.72;

        samples.push((value.clamp(-1.0, 1.0) * i16::MAX as f32) as i16);
    }

    samples
}

fn temple_pad(root: f32, time: f32) -> f32 {
    (TAU * root * 0.5 * time).sin() * 0.23
        + (TAU * root * 0.75 * time).sin() * 0.11
        + (TAU * root * time).sin() * 0.08
}

fn smooth_step(value: f32) -> f32 {
    value * value * (3.0 - 2.0 * value)
}

fn encode_wav(samples: &[i16], sample_rate: u32) -> Vec<u8> {
    let data_size = u32::try_from(std::mem::size_of_val(samples))
        .expect("the generated music must fit in a WAV file");
    let byte_rate = sample_rate * u32::from(CHANNELS) * u32::from(BITS_PER_SAMPLE) / 8;
    let block_align = CHANNELS * BITS_PER_SAMPLE / 8;
    let mut bytes = Vec::with_capacity(44 + data_size as usize);

    bytes.extend_from_slice(b"RIFF");
    bytes.extend_from_slice(&(36 + data_size).to_le_bytes());
    bytes.extend_from_slice(b"WAVE");
    bytes.extend_from_slice(b"fmt ");
    bytes.extend_from_slice(&16_u32.to_le_bytes());
    bytes.extend_from_slice(&1_u16.to_le_bytes());
    bytes.extend_from_slice(&CHANNELS.to_le_bytes());
    bytes.extend_from_slice(&sample_rate.to_le_bytes());
    bytes.extend_from_slice(&byte_rate.to_le_bytes());
    bytes.extend_from_slice(&block_align.to_le_bytes());
    bytes.extend_from_slice(&BITS_PER_SAMPLE.to_le_bytes());
    bytes.extend_from_slice(b"data");
    bytes.extend_from_slice(&data_size.to_le_bytes());
    for sample in samples {
        bytes.extend_from_slice(&sample.to_le_bytes());
    }

    bytes
}

#[cfg(test)]
mod tests {
    use super::{
        BITS_PER_SAMPLE, CHANNELS, MUSIC_DURATION_SECONDS, SAMPLE_RATE,
        compose_background_music_samples, compose_background_music_wav,
    };

    #[test]
    fn generated_music_is_a_non_silent_valid_pcm_wav() {
        let samples = compose_background_music_samples();
        let wav = compose_background_music_wav();
        let expected_samples = (SAMPLE_RATE * MUSIC_DURATION_SECONDS) as usize;
        let data_size = u32::from_le_bytes(wav[40..44].try_into().expect("data size"));

        assert_eq!(samples.len(), expected_samples);
        assert!(samples.iter().any(|sample| sample.unsigned_abs() > 1_000));
        assert_eq!(&wav[0..4], b"RIFF");
        assert_eq!(&wav[8..12], b"WAVE");
        assert_eq!(&wav[12..16], b"fmt ");
        assert_eq!(&wav[36..40], b"data");
        assert_eq!(data_size as usize, samples.len() * 2);
        assert_eq!(wav.len(), 44 + data_size as usize);
        assert_eq!(CHANNELS, 1);
        assert_eq!(BITS_PER_SAMPLE, 16);
    }
}
