const QUAD_CENTER = vec3<f32>(0.0, 0.0, 0.0);

struct CameraUniform {
  view_proj: mat4x4<f32>,
}

struct Screen {
  width: f32,
  height: f32,
}

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

@group(1) @binding(0)
var<uniform> screen: Screen;

struct VertexInput {
  @location(0) position: vec3<f32>,
  @location(1) uv: vec2<f32>,
}

struct InstanceInput {
    @location(2) model_matrix_0: vec4<f32>,
    @location(3) model_matrix_1: vec4<f32>,
    @location(4) model_matrix_2: vec4<f32>,
    @location(5) model_matrix_3: vec4<f32>,
    @location(6) model_color: vec3<f32>,
};


struct VertexOutput {
  @builtin(position) clip_position: vec4<f32>,
  @location(0) uv: vec2<f32>,
  @location(1) color: vec3<f32>,
};

@vertex
fn vs_main(
    model: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {
    let model_matrix = mat4x4<f32>(
        instance.model_matrix_0,
        instance.model_matrix_1,
        instance.model_matrix_2,
        instance.model_matrix_3
    );

    var position: vec4<f32> = camera.view_proj * model_matrix * vec4<f32>(QUAD_CENTER, 1.0);
    position /= position.w;
    position = position
      + vec4<f32>(
        model.position.xy * vec2<f32>(50.0 / screen.width, 50.0 / screen.height),
        0.0,
        0.0
      );

    var out: VertexOutput;
    out.uv = model.uv;
    out.clip_position = position;
    out.color = instance.model_color;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    if (length(in.uv - vec2<f32>(0.5, 0.5)) > 0.5) {
      discard;
    }

    return vec4<f32>(1.0);
}
