use bevy::prelude::*;
use std::collections::HashMap;

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(TerrainMap::new(2048, 1024)) // Much larger terrain
            .add_systems(Startup, setup_terrain)
            .add_systems(Update, update_terrain_mesh);
    }
}

#[derive(Resource)]
pub struct TerrainMap {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<bool>, // true = solid, false = empty
    pub dirty_chunks: HashMap<(i32, i32), bool>,
    pub chunk_size: usize,
}

impl TerrainMap {
    pub fn new(width: usize, height: usize) -> Self {
        let mut pixels = vec![false; width * height];
        
        // Generate varied terrain with multiple layers of hills
        for x in 0..width {
            let x_norm = x as f32 / width as f32;
            
            // Base terrain level (bottom 40% of screen)
            let base_height = (height as f32 * 0.4) as i32;
            
            // Large rolling hills
            let large_hills = ((x_norm * std::f32::consts::PI * 2.0).sin() * 80.0) as i32;
            
            // Medium hills for variation
            let medium_hills = ((x_norm * std::f32::consts::PI * 6.0).sin() * 40.0) as i32;
            
            // Small details
            let small_hills = ((x_norm * std::f32::consts::PI * 20.0).sin() * 15.0) as i32;
            
            // Add some randomness
            let random_offset = (fastrand::f32() * 20.0 - 10.0) as i32;
            
            let terrain_height = (base_height + large_hills + medium_hills + small_hills + random_offset)
                .max(height as i32 / 10) // Minimum height
                .min(height as i32 * 3 / 4) as usize; // Maximum height
            
            // Fill from bottom up to terrain height
            for y in 0..terrain_height.min(height) {
                pixels[y * width + x] = true;
            }
        }
        
        Self {
            width,
            height,
            pixels,
            dirty_chunks: HashMap::new(),
            chunk_size: 64,
        }
    }
    
    pub fn is_solid(&self, x: i32, y: i32) -> bool {
        if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32 {
            return false;
        }
        self.pixels[y as usize * self.width + x as usize]
    }
    
    pub fn destroy_circle(&mut self, center_x: f32, center_y: f32, radius: f32) {
        let min_x = ((center_x - radius) as i32).max(0);
        let max_x = ((center_x + radius) as i32).min(self.width as i32 - 1);
        let min_y = ((center_y - radius) as i32).max(0);
        let max_y = ((center_y + radius) as i32).min(self.height as i32 - 1);
        
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let dx = x as f32 - center_x;
                let dy = y as f32 - center_y;
                let distance = (dx * dx + dy * dy).sqrt();
                
                if distance <= radius {
                    let index = y as usize * self.width + x as usize;
                    if self.pixels[index] {
                        self.pixels[index] = false;
                        
                        // Mark chunk as dirty
                        let chunk_x = x / self.chunk_size as i32;
                        let chunk_y = y / self.chunk_size as i32;
                        self.dirty_chunks.insert((chunk_x, chunk_y), true);
                    }
                }
            }
        }
    }
    
    pub fn check_collision(&self, x: f32, y: f32, radius: f32) -> bool {
        let min_x = ((x - radius) as i32).max(0);
        let max_x = ((x + radius) as i32).min(self.width as i32 - 1);
        let min_y = ((y - radius) as i32).max(0);
        let max_y = ((y + radius) as i32).min(self.height as i32 - 1);
        
        for check_y in min_y..=max_y {
            for check_x in min_x..=max_x {
                if self.is_solid(check_x, check_y) {
                    let dx = check_x as f32 - x;
                    let dy = check_y as f32 - y;
                    let distance = (dx * dx + dy * dy).sqrt();
                    
                    if distance <= radius {
                        return true;
                    }
                }
            }
        }
        false
    }
}

#[derive(Component)]
pub struct TerrainRenderer;

fn setup_terrain(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    terrain: Res<TerrainMap>,
) {
    // Create terrain mesh
    let mesh = create_terrain_mesh(&terrain);
    
    commands.spawn((
        Mesh2d(meshes.add(mesh)),
        MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(0.4, 0.3, 0.2)))),
        Transform::from_translation(Vec3::new(
            -(terrain.width as f32) / 2.0,
            -(terrain.height as f32) / 2.0,
            0.0,
        )),
        TerrainRenderer,
    ));
}

fn update_terrain_mesh(
    mut meshes: ResMut<Assets<Mesh>>,
    mut terrain: ResMut<TerrainMap>,
    mut query: Query<&mut Mesh2d, With<TerrainRenderer>>,
) {
    if !terrain.dirty_chunks.is_empty() {
        terrain.dirty_chunks.clear();
        
        // Regenerate mesh
        let new_mesh = create_terrain_mesh(&terrain);
        
        for mesh2d in query.iter_mut() {
            if let Some(mesh) = meshes.get_mut(&mesh2d.0) {
                *mesh = new_mesh.clone();
            }
        }
    }
}

fn create_terrain_mesh(terrain: &TerrainMap) -> Mesh {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    
    // Simple mesh generation - create quads for solid pixels
    for y in 0..terrain.height {
        for x in 0..terrain.width {
            if terrain.pixels[y * terrain.width + x] {
                let x_f = x as f32;
                let y_f = y as f32;
                
                let vertex_start = vertices.len() as u32;
                
                // Add quad vertices
                vertices.push([x_f, y_f, 0.0]);         // bottom-left
                vertices.push([x_f + 1.0, y_f, 0.0]);  // bottom-right
                vertices.push([x_f + 1.0, y_f + 1.0, 0.0]); // top-right
                vertices.push([x_f, y_f + 1.0, 0.0]);  // top-left
                
                // Add quad indices (two triangles)
                indices.extend_from_slice(&[
                    vertex_start, vertex_start + 1, vertex_start + 2,
                    vertex_start, vertex_start + 2, vertex_start + 3,
                ]);
            }
        }
    }
    
    let mut mesh = Mesh::new(bevy::render::render_resource::PrimitiveTopology::TriangleList, default());
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.insert_indices(bevy::render::mesh::Indices::U32(indices));
    mesh
}