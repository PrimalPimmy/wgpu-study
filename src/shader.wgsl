// circle.wgsl

struct VSOut {
  @builtin(position) pos: vec4<f32>,
  @location(0) v_color: vec3<f32>,
};

@vertex
fn vs_main(
  @location(0) a_position: vec3<f32>,
  @location(1) a_color: vec3<f32>,
) -> VSOut {
  var out: VSOut;
  // Positions are expected to already be in clip space [-1,1]; z set to 0 for 2D
  out.pos = vec4<f32>(a_position, 1.0);
  out.v_color = a_color;
  return out;
}

@fragment
fn fs_main(in: VSOut) -> @location(0) vec4<f32> {
  // If the surface format is sRGB, the GPU will handle conversion from linear
  return vec4<f32>(in.v_color, 1.0);
}