use anyhow::{anyhow, Result};
use js_sys::ArrayBuffer;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{AudioBuffer, AudioBufferSourceNode, AudioContext, AudioDestinationNode, AudioNode};

pub fn create_audio_context() -> Result<AudioContext> {
    AudioContext::new().map_err(|err| anyhow!("Could not create audio context: {:#?}", err))
}

fn create_buffer_source(ctx: &AudioContext) -> Result<AudioBufferSourceNode> {
    ctx.create_buffer_source()
        .map_err(|err| anyhow!("Error creating buffer source: {:#?}", err))
}

fn connect_with_audio_node(
    buffer_source: &AudioBufferSourceNode,
    destination: &AudioDestinationNode,
) -> Result<AudioNode> {
    buffer_source
        .connect_with_audio_node(destination)
        .map_err(|err| anyhow!("Error connecting audio source to destination {:#?}", err))
}

pub fn play_sound(ctx: &AudioContext, buffer: &AudioBuffer) -> Result<()> {
    let track_source = create_buffer_source(ctx)?;
    track_source.set_buffer(Some(&buffer));
    connect_with_audio_node(&track_source, &ctx.destination())?;
    track_source
        .start()
        .map_err(|err| anyhow!("Could not start sound!{:#?}", err))
}

pub async fn decode_audio_data(
    ctx: &AudioContext,
    array_beffer: &ArrayBuffer,
) -> Result<AudioBuffer> {
    JsFuture::from(
        ctx.decode_audio_data(&array_beffer)
            .map_err(|err| anyhow!("Could not decode audio from array buffer {:#?}", err))?,
    )
    .await
    .map_err(|err| anyhow!("Could not convert promise to future {:#?}", err))?
    .dyn_into()
    .map_err(|err| anyhow!("Could not cast into AudioBuffer {:#?}", err))
}
