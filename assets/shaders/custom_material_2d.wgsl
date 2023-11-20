#import bevy_sprite::mesh2d_vertex_output::VertexOutput
// we can import items from shader modules in the assets folder with a quoted path
#import "shaders/custom_material_import.wgsl"::COLOR_MULTIPLIER

@group(1) @binding(0) var<uniform> zoom: f32;
@group(1) @binding(1) var<uniform> center: vec2<f32>;
@group(1) @binding(2) var<uniform> epsilon: f32;



@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
  let uv = mesh.world_position.xy;
  let c = uv  * zoom + center;
  var z = vec2<f32>(0.0, 0.0);
  var i = 0.0f;
  while( z.x * z.x + z.y * z.y < 4.0 && i < epsilon) {
    z = vec2<f32>(z.x * z.x - z.y * z.y, 2.0 * z.x * z.y) + c;
    i += 1.0f;
    }
  let color = i / epsilon;
  return vec4<f32>(color, color, color, 1.0);
}
