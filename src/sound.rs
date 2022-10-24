use anyhow::{anyhow, Result};
use js_sys::ArrayBuffer;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    AudioBuffer, AudioBufferSourceNode, AudioContext, AudioDestinationNode, AudioNode, GainNode,
};

pub async fn decode_audio_data(
    ctx: &AudioContext,
    array_buffer: &ArrayBuffer,
) -> Result<AudioBuffer> {
    JsFuture::from(
        ctx.decode_audio_data(array_buffer)
            .map_err(|err| anyhow!("Could not decode audio from array buffer {:#?}", err))?,
    )
    .await
    .map_err(|err| anyhow!("Could not convert promise to future {:#?}", err))?
    .dyn_into()
    .map_err(|err| anyhow!("Could not cast into AudioBuffer {:#?}", err))
}

pub fn create_audio_context() -> Result<AudioContext> {
    AudioContext::new().map_err(|err| anyhow!("Could not create audio context: {:#?}", err))
}

pub fn create_gain(ctx: &AudioContext) -> Result<GainNode> {
    ctx.create_gain()
        .map_err(|err| anyhow!("Could not create gain: {:#?}", err))
}

fn create_buffer_source(ctx: &AudioContext) -> Result<AudioBufferSourceNode> {
    ctx.create_buffer_source()
        .map_err(|err| anyhow!("Error creating buffer source {:#?}", err))
}

fn connect_with_audio_node_js(
    buffer_source: &AudioBufferSourceNode,
    gain: &GainNode,
    destination: &AudioDestinationNode,
) -> Result<AudioNode, JsValue> {
    buffer_source
        .connect_with_audio_node(gain)?
        .connect_with_audio_node(destination)
}

fn connect_with_audio_node(
    buffer_source: &AudioBufferSourceNode,
    gain: &GainNode,
    destination: &AudioDestinationNode,
) -> Result<AudioNode> {
    connect_with_audio_node_js(buffer_source, gain, destination)
        .map_err(|err| anyhow!("Error connecting audio with gain {:#?}", err))
}

fn create_track_source(
    ctx: &AudioContext,
    buffer: &AudioBuffer,
    volume: f32,
) -> Result<AudioBufferSourceNode> {
    let track_source = create_buffer_source(ctx)?;
    track_source.set_buffer(Some(buffer));
    let gain = create_gain(ctx)?;
    connect_with_audio_node(&track_source, &gain, &ctx.destination())?;
    gain.gain().set_value(volume);

    Ok(track_source)
}

pub enum LOOPING {
    NO,
    Yes,
}

pub fn play_sound(
    ctx: &AudioContext,
    buffer: &AudioBuffer,
    looping: LOOPING,
    volume: f32,
) -> Result<()> {
    let track_source = create_track_source(ctx, buffer, volume)?;
    if matches!(looping, LOOPING::Yes) {
        track_source.set_loop(true);
    }
    track_source
        .start()
        .map_err(|err| anyhow!("Could not start sound!{:#?}", err))
}
