use std::{f32::consts::TAU, sync::OnceLock};

use raylib::prelude::{Music, RaylibAudio, Sound};

const SAMPLE_RATE: u32 = 22_050;
const MUSIC_DURATION_SECONDS: u32 = 16;
const CHANNELS: u16 = 1;
const BITS_PER_SAMPLE: u16 = 16;

static BACKGROUND_MUSIC_WAV: OnceLock<Vec<u8>> = OnceLock::new();
static EFFECT_WAVEFORMS: OnceLock<EffectWaveforms> = OnceLock::new();

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioCue {
    MenuMove,
    MenuConfirm,
    Shot,
    GuardianHit,
    GuardianDefeated,
    KeyCollected,
    PortalActivated,
    Victory,
}

struct EffectWaveforms {
    menu_move: Vec<u8>,
    menu_confirm: Vec<u8>,
    shot: Vec<u8>,
    guardian_hit: Vec<u8>,
    guardian_defeated: Vec<u8>,
    key_collected: Vec<u8>,
    portal_activated: Vec<u8>,
    victory: Vec<u8>,
}

pub struct AudioSystem<'audio> {
    music: Music<'audio>,
    menu_move: Sound<'audio>,
    menu_confirm: Sound<'audio>,
    shot: Sound<'audio>,
    guardian_hit: Sound<'audio>,
    guardian_defeated: Sound<'audio>,
    key_collected: Sound<'audio>,
    portal_activated: Sound<'audio>,
    victory: Sound<'audio>,
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

        let effects = EFFECT_WAVEFORMS.get_or_init(compose_effect_waveforms);
        let menu_move = load_sound(device, &effects.menu_move, "movimiento de menu")?;
        let menu_confirm = load_sound(device, &effects.menu_confirm, "confirmacion de menu")?;
        let shot = load_sound(device, &effects.shot, "disparo")?;
        let guardian_hit = load_sound(device, &effects.guardian_hit, "impacto")?;
        let guardian_defeated =
            load_sound(device, &effects.guardian_defeated, "guardian derrotado")?;
        let key_collected = load_sound(device, &effects.key_collected, "llave")?;
        let portal_activated = load_sound(device, &effects.portal_activated, "portal")?;
        let victory = load_sound(device, &effects.victory, "victoria")?;

        for sound in [
            &menu_move,
            &menu_confirm,
            &shot,
            &guardian_hit,
            &guardian_defeated,
            &key_collected,
            &portal_activated,
            &victory,
        ] {
            sound.set_volume(crate::config::SOUND_EFFECT_VOLUME);
        }

        Ok(Self {
            music,
            menu_move,
            menu_confirm,
            shot,
            guardian_hit,
            guardian_defeated,
            key_collected,
            portal_activated,
            victory,
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

    pub fn play(&self, cue: AudioCue) {
        if !self.enabled {
            return;
        }

        match cue {
            AudioCue::MenuMove => self.menu_move.play(),
            AudioCue::MenuConfirm => self.menu_confirm.play(),
            AudioCue::Shot => self.shot.play(),
            AudioCue::GuardianHit => self.guardian_hit.play(),
            AudioCue::GuardianDefeated => self.guardian_defeated.play(),
            AudioCue::KeyCollected => self.key_collected.play(),
            AudioCue::PortalActivated => self.portal_activated.play(),
            AudioCue::Victory => self.victory.play(),
        }
    }
}

fn load_sound<'audio>(
    device: &'audio RaylibAudio,
    bytes: &[u8],
    label: &str,
) -> Result<Sound<'audio>, String> {
    let wave = device
        .new_wave_from_memory(".wav", bytes)
        .map_err(|error| format!("no se pudo decodificar el efecto de {label}: {error}"))?;
    device
        .new_sound_from_wave(&wave)
        .map_err(|error| format!("no se pudo cargar el efecto de {label}: {error}"))
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

fn compose_effect_waveforms() -> EffectWaveforms {
    EffectWaveforms {
        menu_move: synthesize_effect(0.09, |time, progress, _| {
            let frequency = 440.0 + progress * 260.0;
            (TAU * frequency * time).sin() * (1.0 - progress).powi(2) * 0.7
        }),
        menu_confirm: synthesize_effect(0.2, |time, progress, _| {
            let envelope = (progress * std::f32::consts::PI).sin();
            ((TAU * 523.25 * time).sin() + 0.55 * (TAU * 783.99 * time).sin()) * envelope * 0.4
        }),
        shot: synthesize_effect(0.18, |time, progress, frame| {
            let envelope = (1.0 - progress).powi(3);
            let sweep = (TAU * (220.0 - progress * 150.0) * time).sin();
            (sweep * 0.65 + deterministic_noise(frame) * 0.55) * envelope
        }),
        guardian_hit: synthesize_effect(0.14, |time, progress, frame| {
            let envelope = (1.0 - progress).powi(2);
            ((TAU * 92.0 * time).sin() * 0.65 + deterministic_noise(frame) * 0.35) * envelope
        }),
        guardian_defeated: synthesize_effect(0.72, |time, progress, frame| {
            let frequency = 170.0 - progress * 115.0;
            let rumble = (TAU * frequency * time).sin() * 0.62;
            let crackle = deterministic_noise(frame) * (1.0 - progress) * 0.28;
            (rumble + crackle) * (1.0 - progress).sqrt()
        }),
        key_collected: synthesize_effect(0.52, |time, progress, _| {
            let frequencies = [659.25_f32, 783.99, 987.77, 1_318.51];
            stepped_chime(time, progress, &frequencies) * 0.72
        }),
        portal_activated: synthesize_effect(0.9, |time, progress, _| {
            let rising = 180.0 + progress * 760.0;
            let shimmer = (TAU * rising * time).sin()
                + 0.4 * (TAU * rising * 1.51 * time).sin()
                + 0.2 * (TAU * 8.0 * time).sin();
            shimmer * (progress * std::f32::consts::PI).sin() * 0.42
        }),
        victory: synthesize_effect(1.45, |time, progress, _| {
            let frequencies = [
                392.0_f32, 493.88, 587.33, 783.99, 587.33, 659.25, 783.99, 987.77,
            ];
            let melody = stepped_chime(time, progress, &frequencies);
            let chord = ((TAU * 196.0 * time).sin() + 0.6 * (TAU * 293.66 * time).sin())
                * (progress * std::f32::consts::PI).sin();
            melody * 0.58 + chord * 0.16
        }),
    }
}

fn synthesize_effect(
    duration_seconds: f32,
    mut generator: impl FnMut(f32, f32, u32) -> f32,
) -> Vec<u8> {
    let frame_count = (SAMPLE_RATE as f32 * duration_seconds).round() as usize;
    let samples = (0..frame_count)
        .map(|frame| {
            let time = frame as f32 / SAMPLE_RATE as f32;
            let progress = frame as f32 / frame_count as f32;
            let value = generator(time, progress, frame as u32).clamp(-1.0, 1.0);
            (value * i16::MAX as f32) as i16
        })
        .collect::<Vec<_>>();
    encode_wav(&samples, SAMPLE_RATE)
}

fn stepped_chime(time: f32, progress: f32, frequencies: &[f32]) -> f32 {
    let scaled = progress * frequencies.len() as f32;
    let note_index = (scaled.floor() as usize).min(frequencies.len() - 1);
    let note_progress = scaled.fract();
    let envelope = (note_progress * std::f32::consts::PI).sin().powi(2);
    let frequency = frequencies[note_index];
    ((TAU * frequency * time).sin() + 0.3 * (TAU * frequency * 2.01 * time).sin()) * envelope
}

fn deterministic_noise(frame: u32) -> f32 {
    let mut value = frame.wrapping_add(0x9e37_79b9);
    value ^= value << 13;
    value ^= value >> 17;
    value ^= value << 5;
    (value & 0xffff) as f32 / 32_767.5 - 1.0
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
        compose_background_music_samples, compose_background_music_wav, compose_effect_waveforms,
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

    #[test]
    fn every_generated_effect_is_a_non_silent_pcm_wav() {
        let effects = compose_effect_waveforms();

        for wav in [
            effects.menu_move,
            effects.menu_confirm,
            effects.shot,
            effects.guardian_hit,
            effects.guardian_defeated,
            effects.key_collected,
            effects.portal_activated,
            effects.victory,
        ] {
            assert_eq!(&wav[0..4], b"RIFF");
            assert_eq!(&wav[8..12], b"WAVE");
            assert_eq!(&wav[36..40], b"data");
            assert!(
                wav[44..].chunks_exact(2).any(|bytes| {
                    i16::from_le_bytes([bytes[0], bytes[1]]).unsigned_abs() > 1_000
                })
            );
        }
    }
}
