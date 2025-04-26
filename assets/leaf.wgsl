#import bevy_pbr::{
    pbr_functions::alpha_discard,
    pbr_fragment::pbr_input_from_standard_material,
}

#ifdef PREPASS_PIPELINE
#import bevy_pbr::{
    prepass_io::{VertexOutput, FragmentOutput},
    pbr_deferred_functions::deferred_output,
}
#else
#import bevy_pbr::{
    forward_io::{VertexOutput, FragmentOutput},
    pbr_functions::{apply_pbr_lighting, main_pass_post_lighting_processing},
    pbr_types::STANDARD_MATERIAL_FLAGS_UNLIT_BIT,
}
#endif

@group(2) @binding(100) var<uniform> material_color: vec4<f32>;
@group(2) @binding(101) var material_color_texture: texture_2d<f32>;
@group(2) @binding(102) var material_color_sampler: sampler;
@group(2) @binding(103) var noise_texture: texture_2d<f32>;
@group(2) @binding(104) var noise_sampler: sampler;

fn csin(x: f32) -> f32 {
    return sin(x) + (sin(x * 2) / 2.) + (sin(x * 4.) / 4.);
}

fn height_noise(x: f32, z: f32, amplitude: f32, freq: f32) -> f32 {
    return (csin(x * freq) + csin(z * freq)) * amplitude;
}

@fragment
fn fragment(
    in: VertexOutput,
    @builtin(front_facing) is_front: bool,
) -> FragmentOutput {
    var pbr_input = pbr_input_from_standard_material(in, is_front);

    let alpha = textureSample(material_color_texture, material_color_sampler, in.uv).r;
    if (alpha == 0.) {
        discard;
    }
    //let color = vec4<f32>(0.2, 0.4, 0.2, 1.);
    let color = textureSample(noise_texture, noise_sampler, in.uv);
    pbr_input.material.base_color = alpha_discard(pbr_input.material, color);

#ifdef PREPASS_PIPELINE
    let out = deferred_output(in, pbr_input);
#else
    var out: FragmentOutput;
    out.color = apply_pbr_lighting(pbr_input);
    out.color = main_pass_post_lighting_processing(pbr_input, out.color);
#endif

    return out;
}