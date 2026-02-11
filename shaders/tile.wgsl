struct VertexInput {
  @location(0) position: vec3<f32>,
  @location(1) tex_coords: vec2<f32>,
}

struct VertexOutput {
  @builtin(position) clip_position: vec4<f32>,
  @location(0) tex_coords: vec2<f32>,
}

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
  var out: VertexOutput;
  out.tex_coords = input.tex_coords;
  out.clip_position = vec4<f32>(input.position, 1.0);
  return out;
}

@group(0) @binding(0) var texture: texture_2d<f32>;
@group(0) @binding(1) var t_sampler: sampler;


@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
  return textureSample(texture, t_sampler, input.tex_coords);
}