use std::collections::VecDeque;

use macroquad::{
    color::Color,
    math::Vec2,
    shapes::{draw_line, draw_triangle},
    window::{screen_height, screen_width},
};

use crate::{
    NEAR, Vector3,
    matrix::{
        Mat4x4, cross_product, dot_product, mat_multiply, mult_vec_mat, rotate_x, rotate_y,
        rotate_z, translate, triangle_clip_plane, vec_div, vec_sub,
    },
};

pub type Vertex = Vector3;

#[derive(Debug, Clone, Copy)]
pub struct Triangle {
    pub vertices: [Vertex; 3],
    pub intensity: f32,
}

impl Triangle {
    fn new(vertices: [Vertex; 3], intensity: f32) -> Self {
        Triangle {
            vertices,
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
            // Transform vertices -> Rotation, Translation, Scale (Not yet implemented)
            let mut transformed_vertices = Vec::with_capacity(3);
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

            // Check if face is visible
            //
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
                let intensity = light_dot * 205.0 + 50.0;

                // Convert World space -> View space
                let view_triangle = Triangle::new(
                    [
                        mult_vec_mat(v1, view_mat),
                        mult_vec_mat(v2, view_mat),
                        mult_vec_mat(v3, view_mat),
                    ],
                    intensity,
                );

                // Clipping triangles against near plane
                let mut clipped_triangles = Vec::with_capacity(2);

                let _num_clipped_triangles = triangle_clip_plane(
                    &Vector3::forward(),
                    &Vector3::new(0.0, 0.0, NEAR),
                    &view_triangle,
                    &mut clipped_triangles,
                );

                for triangle in clipped_triangles {
                    // Project to screen: 3D -> 2D
                    let mut projected_vertices = Vec::with_capacity(3);

                    for i in &triangle.vertices {
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
                        [
                            projected_vertices[0],
                            projected_vertices[1],
                            projected_vertices[2],
                        ],
                        intensity,
                    ));
                }
            }
        }

        // Sort triangles by average depth (painter's algorithm)
        // Render triangles in order of highest depth (z-index) to lowest
        triangles_to_raster.sort_by(|t1, t2| {
            let z1 = (t1.vertices[0].z + t1.vertices[1].z + t1.vertices[2].z) / 3.0;
            let z2 = (t2.vertices[0].z + t2.vertices[1].z + t2.vertices[2].z) / 3.0;

            z1.partial_cmp(&z2).unwrap()
        });

        for triangle in triangles_to_raster {
            // Clip triangle against screen boundaries
            let mut clipped_triangles = Vec::with_capacity(2);
            let mut triangle_queue = VecDeque::new();

            triangle_queue.push_back(triangle);
            let mut triangles_to_check = 1;

            // For every side
            for i in 0..4 {
                while triangles_to_check > 0 {
                    let triangle = triangle_queue.pop_front().unwrap();
                    triangles_to_check -= 1;
                    match i {
                        0 => {
                            // Top plane
                            triangle_clip_plane(
                                &Vector3::up(),
                                &Vector3::new(0.0, 10.0, 0.0),
                                &triangle,
                                &mut clipped_triangles,
                            );
                        }
                        1 => {
                            // Bottom plane
                            triangle_clip_plane(
                                &Vector3::down(),
                                &Vector3::new(0.0, screen_height() - 10.0, 0.0),
                                &triangle,
                                &mut clipped_triangles,
                            );
                        }
                        2 => {
                            // Left plane
                            triangle_clip_plane(
                                &Vector3::right(),
                                &Vector3::new(10.0, 0.0, 0.0),
                                &triangle,
                                &mut clipped_triangles,
                            );
                        }
                        3 => {
                            // Right plane
                            triangle_clip_plane(
                                &Vector3::left(),
                                &Vector3::new(screen_width() - 10.0, 0.0, 0.0),
                                &triangle,
                                &mut clipped_triangles,
                            );
                        }
                        _ => {}
                    }
                    for triangle in clipped_triangles.drain(..) {
                        triangle_queue.push_back(triangle);
                    }
                }
                triangles_to_check = triangle_queue.len();
            }

            for clipped_triangle in triangle_queue {
                Self::draw_triangle_face(clipped_triangle);
                Self::draw_triangle_wireframe(clipped_triangle);
            }
        }
    }

    fn draw_triangle_face(triangle: Triangle) {
        let color_value = triangle.intensity.clamp(50.0, 255.0) as u8;
        // Draw face
        draw_triangle(
            Vec2::new(triangle.vertices[0].x, triangle.vertices[0].y),
            Vec2::new(triangle.vertices[1].x, triangle.vertices[1].y),
            Vec2::new(triangle.vertices[2].x, triangle.vertices[2].y),
            Color::from_rgba(color_value, color_value, color_value, 255),
        );
    }

    fn draw_triangle_wireframe(triangle: Triangle) {
        // Draw wireframe
        draw_line(
            triangle.vertices[0].x,
            triangle.vertices[0].y,
            triangle.vertices[1].x,
            triangle.vertices[1].y,
            1.0,
            Color::from_rgba(0, 0, 0, 255),
        );
        draw_line(
            triangle.vertices[1].x,
            triangle.vertices[1].y,
            triangle.vertices[2].x,
            triangle.vertices[2].y,
            1.0,
            Color::from_rgba(0, 0, 0, 255),
        );
        draw_line(
            triangle.vertices[2].x,
            triangle.vertices[2].y,
            triangle.vertices[0].x,
            triangle.vertices[0].y,
            1.0,
            Color::from_rgba(0, 0, 0, 255),
        );
    }
}
