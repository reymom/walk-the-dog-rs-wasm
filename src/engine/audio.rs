use crate::browser;
use crate::engine::{Audio, Sound};
use crate::sound;
use anyhow::Result;

impl Audio {
    pub fn new() -> Result<Self> {
        let context = sound::create_audio_context()?;
        Ok(Audio { context })
    }

    pub async fn load_sound(&self, filename: &str) -> Result<Sound> {
        let array_buffer = browser::fetch_array_buffer(filename).await?;
        let audio_buffer = sound::decode_audio_data(&self.context, &array_buffer).await?;
        Ok(Sound {
            buffer: audio_buffer,
        })
    }

    pub fn play_sound(&self, sound: &Sound) -> Result<()> {
        sound::play_sound(&self.context, &sound.buffer, sound::LOOPING::NO, 1.0)
    }

    pub fn play_looping_sound(&self, sound: &Sound, volume: f32) -> Result<()> {
        sound::play_sound(&self.context, &sound.buffer, sound::LOOPING::YES, volume)
    }
}
