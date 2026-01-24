use macroquad::{color::Color, math::Vec2, shapes::draw_triangle};

use crate::{
    Vector3,
    matrix::{
        Mat4x4, cross_product, dot_product, mat_multiply, mult_vec_mat, rotate_x, rotate_y,
        rotate_z, translate, vec_div, vec_sub,
    },
};

pub type Vertex = Vector3;

struct Triangle {
    v1: Vertex,
    v2: Vertex,
    v3: Vertex,
    intensity: f32,
}

impl Triangle {
    fn new(v1: Vertex, v2: Vertex, v3: Vertex, intensity: f32) -> Self {
        Triangle {
            v1,
            v2,
            v3,
            intensity,
        }
    }
}

pub struct Mesh {
    vertices: Vec<Vertex>,
    faces: Vec<[usize; 3]>,
}

impl Mesh {
    pub fn load_from_obj(path: &str) -> Result<Self, std::io::Error> {
        let mut vertices = Vec::new();
        let mut faces = Vec::new();

        let contents = std::fs::read_to_string(path)?;
        let lines = contents.lines();

        for line in lines {
            match line.get(0..1).unwrap_or_default() {
                "v" => {
                    let parts: Vec<&str> = line[2..].split_whitespace().collect();
                    let vertex = Vector3::new(
                        parts[0].parse().unwrap(),
                        parts[1].parse().unwrap(),
                        parts[2].parse().unwrap(),
                    );

                    vertices.push(vertex)
                }
                "f" => {
                    let parts: Vec<&str> = line[2..].split_whitespace().collect();
                    let face = [
                        parts[0].parse::<usize>().unwrap() - 1,
                        parts[1].parse::<usize>().unwrap() - 1,
                        parts[2].parse::<usize>().unwrap() - 1,
                    ];
                    faces.push(face)
                }
                _ => {}
            }
        }

        Ok(Mesh { vertices, faces })
    }

    pub fn draw(
        &self,
        width: f32,
        height: f32,
        rotation: &Vector3,
        translation: &Vector3,
        view_mat: &Mat4x4,
        camera_position: &Vector3,
        light_direction: &Vector3,
        projection_mat: &Mat4x4,
    ) {
        let mut triangles_to_raster = Vec::new();
        for face in &self.faces {
            let mut transformed_vertices = Vec::with_capacity(3);
            // Transform vertices -> Rotation, Translation, Scale (Not yet implemented)
            for v in face {
                let vertex = &self.vertices[*v];

                // Rotate
                let transform_mat = mat_multiply(&rotate_x(rotation.x), &rotate_y(rotation.y));
                let transform_mat = mat_multiply(&transform_mat, &rotate_z(rotation.z));

                // Translate
                let transform_mat = mat_multiply(
                    &transform_mat,
                    &translate(translation.x, translation.y, translation.z),
                );

                let transformed = mult_vec_mat(&vertex, &transform_mat);

                transformed_vertices.push(transformed);
            }

            let v1 = &transformed_vertices[0];
            let v2 = &transformed_vertices[1];
            let v3 = &transformed_vertices[2];

            // Calculate the normal vector
            let line1 = vec_sub(v2, v1);
            let line2 = vec_sub(v3, v1);

            let normal = cross_product(&line1, &line2).normalize();

            // From camera to the normal -> Check if face is visible
            let ray = vec_sub(v1, camera_position);
            let normal_dot = dot_product(&normal, &ray);

            // Render only if visible
            if normal_dot < 0.0 {
                // Calculate light intensity
                let light_dot = dot_product(&normal, &light_direction);
                // Minimum intensity at 10
                let intensity = light_dot * 205.0 + 50.0;

                // Convert World space to View space
                let mut view_vertices = Vec::with_capacity(3);

                view_vertices.push(mult_vec_mat(v1, view_mat));
                view_vertices.push(mult_vec_mat(v2, view_mat));
                view_vertices.push(mult_vec_mat(v3, view_mat));

                // Project to screen
                let mut projected_vertices = Vec::with_capacity(3);
                for i in &view_vertices {
                    // Project to screen
                    let projected = mult_vec_mat(i, projection_mat);
                    // Normalize into cartesian coordinates using w component
                    let mut projected = vec_div(&projected, projected.w);

                    // Scale to screen dimensions
                    projected.x = (projected.x + 1.0) * width / 2.0;
                    projected.y = (projected.y + 1.0) * height / 2.0;

                    projected_vertices.push(projected);
                }

                triangles_to_raster.push(Triangle::new(
                    projected_vertices[0],
                    projected_vertices[1],
                    projected_vertices[2],
                    intensity,
                ));
            }
        }

        // Sort triangles by average depth (painter's algorithm)
        triangles_to_raster.sort_by(|t1, t2| {
            let z1 = (t1.v1.z + t1.v2.z + t1.v3.z) / 3.0;
            let z2 = (t2.v1.z + t2.v2.z + t2.v3.z) / 3.0;

            z1.partial_cmp(&z2).unwrap()
        });

        // Render triangles in order of highest depth (z-index) to lowest
        for triangle in triangles_to_raster {
            let color_value = triangle.intensity.clamp(0.0, 255.0) as u8;
            draw_triangle(
                Vec2::new(triangle.v1.x, triangle.v1.y),
                Vec2::new(triangle.v2.x, triangle.v2.y),
                Vec2::new(triangle.v3.x, triangle.v3.y),
                Color::from_rgba(color_value, color_value, color_value, 255),
            );
            // draw_line(
            //     projected_vertices[0].x,
            //     projected_vertices[0].y,
            //     projected_vertices[1].x,
            //     projected_vertices[1].y,
            //     1.0,
            //     Color::from_rgba(255, 255, 255, 255),
            // );
            // draw_line(
            //     projected_vertices[1].x,
            //     projected_vertices[1].y,
            //     projected_vertices[2].x,
            //     projected_vertices[2].y,
            //     1.0,
            //     Color::from_rgba(255, 255, 255, 255),
            // );
            // draw_line(
            //     projected_vertices[2].x,
            //     projected_vertices[2].y,
            //     projected_vertices[0].x,
            //     projected_vertices[0].y,
            //     1.0,
            //     Color::from_rgba(255, 255, 255, 255),
            // );
        }
    }
}
