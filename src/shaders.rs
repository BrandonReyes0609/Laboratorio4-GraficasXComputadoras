use nalgebra_glm::{Vec3, Vec4, Mat3, mat4_to_mat3};
use crate::vertex::Vertex;
use crate::Uniforms;
use crate::fragment::Fragment;
use crate::color::Color;
use rand::Rng;
use rand::SeedableRng;
use rand::rngs::StdRng;

pub fn vertex_shader(vertex: &Vertex, uniforms: &Uniforms) -> Vertex {
  // Transform position
  let position = Vec4::new(
    vertex.position.x,
    vertex.position.y,
    vertex.position.z,
    1.0
  );
  let transformed = uniforms.projection_matrix * uniforms.view_matrix * uniforms.model_matrix * position;

  // Perform perspective division
  let w = transformed.w;
  let ndc_position = Vec4::new(
    transformed.x / w,
    transformed.y / w,
    transformed.z / w,
    1.0
  );

  // apply viewport matrix
  let screen_position = uniforms.viewport_matrix * ndc_position;

  // Transform normal
  let model_mat3 = mat4_to_mat3(&uniforms.model_matrix); 
  let normal_matrix = model_mat3.transpose().try_inverse().unwrap_or(Mat3::identity());

  let transformed_normal = normal_matrix * vertex.normal;

  // Create a new Vertex with transformed attributes
  Vertex {
    position: vertex.position,
    normal: vertex.normal,
    tex_coords: vertex.tex_coords,
    color: vertex.color,
    transformed_position: Vec3::new(screen_position.x, screen_position.y, screen_position.z),
    transformed_normal,
  }
}

pub fn fragment_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  // random_color_shader(fragment, uniforms)
  // black_and_white(fragment, uniforms)
  //  (fragment, uniforms)
  // cloud_shader(fragment, uniforms)
  // cellular_shader(fragment, uniforms)
  // cracked_ground_shader(fragment, uniforms)
  //lava_shader(fragment, uniforms)
  star(fragment, uniforms)

}

fn random_color_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  let seed = uniforms.time as u64;

  let mut rng = StdRng::seed_from_u64(seed);

  let r = rng.gen_range(0..=255);
  let g = rng.gen_range(0..=255);
  let b = rng.gen_range(0..=255);

  let random_color = Color::new(r, g, b);

  random_color * fragment.intensity
}

fn black_and_white(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  let seed = uniforms.time as f32 * fragment.vertex_position.y * fragment.vertex_position.x;

  let mut rng = StdRng::seed_from_u64(seed.abs() as u64);

  let random_number = rng.gen_range(0..=100);

  let black_or_white = if random_number < 50 {
    Color::new(0, 0, 0)
  } else {
    Color::new(255, 255, 255)
  };

  black_or_white * fragment.intensity
}

pub fn dalmata_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  let zoom = 100.0;
  let ox = 0.0;
  let oy = 0.0;
  let x = fragment.vertex_position.x;
  let y = fragment.vertex_position.y;

  let noise_value = uniforms.noise.get_noise_2d(
    (x + ox) * zoom,
    (y + oy) * zoom,
  );

  let spot_threshold = 0.5;
  let spot_color = Color::new(255, 255, 255); // White
  let base_color = Color::new(0, 0, 0); // Black

  let noise_color = if noise_value < spot_threshold {
    spot_color
  } else {
    base_color
  };

  noise_color * fragment.intensity
}

fn cloud_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  let zoom = 100.0;  // to move our values 
  let ox = 100.0; // offset x in the noise map
  let oy = 100.0;
  let x = fragment.vertex_position.x;
  let y = fragment.vertex_position.y;
  let t = uniforms.time as f32 * 0.5;

  let noise_value = uniforms.noise.get_noise_2d(x * zoom + ox + t, y * zoom + oy);

  // Define cloud threshold and colors
  let cloud_threshold = 0.5; // Adjust this value to change cloud density
  let cloud_color = Color::new(255, 255, 255); // White for clouds
  let sky_color = Color::new(30, 97, 145); // Sky blue

  // Determine if the pixel is part of a cloud or sky
  let noise_color = if noise_value > cloud_threshold {
    cloud_color
  } else {
    sky_color
  };

  noise_color * fragment.intensity
}

fn cellular_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  let zoom = 30.0;  // Zoom factor to adjust the scale of the cell pattern
  let ox = 50.0;    // Offset x in the noise map
  let oy = 50.0;    // Offset y in the noise map
  let x = fragment.vertex_position.x;
  let y = fragment.vertex_position.y;

  // Use a cellular noise function to create the plant cell pattern
  let cell_noise_value = uniforms.noise.get_noise_2d(x * zoom + ox, y * zoom + oy).abs();

  // Define different shades of green for the plant cells
  let cell_color_1 = Color::new(85, 107, 47);   // Dark olive green
  let cell_color_2 = Color::new(124, 252, 0);   // Light green
  let cell_color_3 = Color::new(34, 139, 34);   // Forest green
  let cell_color_4 = Color::new(173, 255, 47);  // Yellow green

  // Use the noise value to assign a different color to each cell
  let final_color = if cell_noise_value < 0.15 {
    cell_color_1
  } else if cell_noise_value < 0.7 {
    cell_color_2
  } else if cell_noise_value < 0.75 {
    cell_color_3
  } else {
    cell_color_4
  };

  // Adjust intensity to simulate lighting effects (optional)
  final_color * fragment.intensity
}

pub fn star(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  // Colores base para el efecto de la estrella
  //let bright_color = Color::new(255, 240, 0); // Naranja brillante (efecto lava)
  let bright_color = Color::new(255, 253, 190); // Color blanco

  let dark_color = Color::new(255, 193, 108);   // Naranja rojizo oscuro

  // Obtener la posición del fragmento
  let position = Vec3::new(
    fragment.vertex_position.x,
    fragment.vertex_position.y,
    fragment.depth
  );

  // Frecuencia base y amplitud para el efecto de pulsación
  let base_frequency = 0.6;
  let pulsate_amplitude = 0.5;
  let t = uniforms.time as f32 * 0.01;

  // Pulsación en el eje z para cambiar el tamaño del punto
  let pulsate = (t * base_frequency).sin() * pulsate_amplitude;

  // Aplicar ruido a las coordenadas con una pulsación sutil en el eje z
  let zoom = 1000.0; // Factor de zoom constante
  let noise_value1 = uniforms.noise.get_noise_3d(
    position.x * zoom,
    position.y * zoom,
    (position.z + pulsate) * zoom
  );
  let noise_value2 = uniforms.noise.get_noise_3d(
    (position.x + 1000.0) * zoom,
    (position.y + 1000.0) * zoom,
    (position.z + 1000.0 + pulsate) * zoom
  );
  let noise_value = (noise_value1 + noise_value2) * 0.5;  // Promediar el ruido para transiciones más suaves

  // Usar interpolación lineal (lerp) para mezclar colores según el valor del ruido
  let color = dark_color.lerp(&bright_color, noise_value);

  color * fragment.intensity
}