struct VertexInput {
  @location(0) position: vec3<f32>,
  @location(1) uv: vec2<f32>,
}

struct VertexOutput {
  @builtin(position) clip_position: vec4<f32>,
  @location(0) uv: vec2<f32>,
};

@vertex
fn vs_main(
  model: VertexInput,
) -> VertexOutput {
  var out: VertexOutput;
  out.uv = model.uv;
  out.clip_position = vec4<f32>(model.position, 1.0);
  return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
  if length(in.uv - vec2<f32>(0.5, 0.5)) <= 0.5 {
    return vec4<f32>(1.0, 1.0, 1.0, 1.0);
  } else {
    discard;
  }
}
