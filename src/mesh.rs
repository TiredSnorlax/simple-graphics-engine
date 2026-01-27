use std::collections::VecDeque;

use macroquad::{
    color::Color,
    math::Vec2,
    shapes::{draw_line, draw_triangle},
    texture::Image,
    window::{screen_height, screen_width},
};

use crate::{
    NEAR, Vector3,
    matrix::{
        Mat4x4, Vector2, cross_product, dot_product, mat_multiply, mult_vec_mat, rotate_x,
        rotate_y, rotate_z, translate, triangle_clip_plane, vec_div, vec_sub, vec2_div,
    },
};

pub type Vertex = Vector3;

#[derive(Debug, Clone, Copy)]
pub struct Triangle {
    pub vertices: [Vertex; 3],
    pub intensity: f32,
    pub texture_coords: [Vector2; 3],
}

impl Triangle {
    fn new(vertices: [Vertex; 3], intensity: f32, texture_coords: [Vector2; 3]) -> Self {
        Triangle {
            vertices,
            intensity,
            texture_coords,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Face {
    pub vertices: [usize; 3],
    pub texture_coords: [Vector2; 3],
}

impl Face {
    fn new(vertices: [usize; 3], texture_coords: [Vector2; 3]) -> Self {
        Face {
            vertices,
            texture_coords,
        }
    }
}

pub struct Mesh {
    vertices: Vec<Vertex>,
    faces: Vec<Face>,
}

impl Mesh {
    pub fn load_from_obj(path: &str, has_texture: bool) -> Result<Self, std::io::Error> {
        let mut vertices = Vec::new();
        let mut faces = Vec::new();
        let mut texture_coords = Vec::new();

        let contents = std::fs::read_to_string(path)?;
        let lines = contents.lines();

        for line in lines {
            match line.get(0..2).unwrap_or_default() {
                "v " => {
                    let parts: Vec<&str> = line[2..].split_whitespace().collect();
                    let vertex = Vector3::new(
                        parts[0].parse().unwrap(),
                        parts[1].parse().unwrap(),
                        parts[2].parse().unwrap(),
                    );

                    vertices.push(vertex)
                }
                "vt" => {
                    // Eg: vt 0.5 0.5
                    let parts: Vec<&str> = line[3..].split_whitespace().collect();
                    let t_c = Vector2::new(
                        parts[0].parse::<f32>().unwrap(),
                        1.0 - parts[1].parse::<f32>().unwrap(),
                    );

                    texture_coords.push(t_c);
                }
                "f " => {
                    // Eg: f 1/1 2/2 3/3 (if has_texture) else f 1 2 3
                    let parts: Vec<&str> = line[2..].split_whitespace().collect();
                    if has_texture {
                        let face_data = parts
                            .iter()
                            .map(|part| {
                                let mut split = part.split('/');
                                let vertex = split.next().unwrap().parse::<usize>().unwrap() - 1;
                                let texture = split.next().unwrap().parse::<usize>().unwrap() - 1;
                                (vertex, texture)
                            })
                            .collect::<Vec<_>>();

                        let face = Face::new(
                            [face_data[0].0, face_data[1].0, face_data[2].0],
                            [
                                texture_coords[face_data[0].1],
                                texture_coords[face_data[1].1],
                                texture_coords[face_data[2].1],
                            ],
                        );
                        faces.push(face);
                        // Some f data has 4 vertices => Split it into two triangles
                        if face_data.len() == 4 {
                            let face_2 = Face::new(
                                [face_data[2].0, face_data[3].0, face_data[0].0],
                                [
                                    texture_coords[face_data[2].1],
                                    texture_coords[face_data[3].1],
                                    texture_coords[face_data[0].1],
                                ],
                            );
                            faces.push(face_2);
                        }
                    } else {
                        let vertices = parts
                            .iter()
                            .map(|part| part.parse::<usize>().unwrap() - 1)
                            .collect::<Vec<_>>();

                        let face = Face::new(
                            [vertices[0], vertices[1], vertices[2]],
                            [
                                Vector2::new(0.0, 0.0),
                                Vector2::new(1.0, 0.0),
                                Vector2::new(1.0, 1.0),
                            ],
                        );
                        faces.push(face);

                        if parts.len() == 4 {
                            let face_2 = Face::new(
                                [vertices[2], vertices[3], vertices[0]],
                                [
                                    Vector2::new(0.0, 0.0),
                                    Vector2::new(1.0, 0.0),
                                    Vector2::new(1.0, 1.0),
                                ],
                            );
                            faces.push(face_2);
                        }
                    }
                }
                _ => {}
            }
        }

        Ok(Mesh { vertices, faces })
    }

    pub fn cube() -> Self {
        let vertices = vec![
            Vector3::new(0.0, 0.0, 0.0), // 0
            Vector3::new(0.0, 1.0, 0.0), // 1
            Vector3::new(1.0, 1.0, 0.0), // 2
            Vector3::new(1.0, 0.0, 0.0), // 3
            Vector3::new(1.0, 1.0, 1.0), // 4
            Vector3::new(1.0, 0.0, 1.0), // 5
            Vector3::new(0.0, 1.0, 1.0), // 6
            Vector3::new(0.0, 0.0, 1.0), // 7
        ];

        let faces = vec![
            // SOUTH
            Face::new(
                [0, 1, 2],
                [
                    Vector2::new(0.0, 1.0),
                    Vector2::new(0.0, 0.0),
                    Vector2::new(1.0, 0.0),
                ],
            ),
            Face::new(
                [0, 2, 3],
                [
                    Vector2::new(0.0, 1.0),
                    Vector2::new(1.0, 0.0),
                    Vector2::new(1.0, 1.0),
                ],
            ),
            // EAST
            Face::new(
                [3, 2, 4],
                [
                    Vector2::new(0.0, 1.0),
                    Vector2::new(0.0, 0.0),
                    Vector2::new(1.0, 0.0),
                ],
            ),
            Face::new(
                [3, 4, 5],
                [
                    Vector2::new(0.0, 1.0),
                    Vector2::new(1.0, 0.0),
                    Vector2::new(1.0, 1.0),
                ],
            ),
            // NORTH
            Face::new(
                [5, 4, 6],
                [
                    Vector2::new(0.0, 1.0),
                    Vector2::new(0.0, 0.0),
                    Vector2::new(1.0, 0.0),
                ],
            ),
            Face::new(
                [5, 6, 7],
                [
                    Vector2::new(0.0, 1.0),
                    Vector2::new(1.0, 0.0),
                    Vector2::new(1.0, 1.0),
                ],
            ),
            // WEST
            Face::new(
                [7, 6, 1],
                [
                    Vector2::new(0.0, 1.0),
                    Vector2::new(0.0, 0.0),
                    Vector2::new(1.0, 0.0),
                ],
            ),
            Face::new(
                [7, 1, 0],
                [
                    Vector2::new(0.0, 1.0),
                    Vector2::new(1.0, 0.0),
                    Vector2::new(1.0, 1.0),
                ],
            ),
            // TOP
            Face::new(
                [1, 6, 4],
                [
                    Vector2::new(0.0, 1.0),
                    Vector2::new(0.0, 0.0),
                    Vector2::new(1.0, 0.0),
                ],
            ),
            Face::new(
                [1, 4, 2],
                [
                    Vector2::new(0.0, 1.0),
                    Vector2::new(1.0, 0.0),
                    Vector2::new(1.0, 1.0),
                ],
            ),
            // BOTTOM
            Face::new(
                [5, 7, 0],
                [
                    Vector2::new(0.0, 1.0),
                    Vector2::new(0.0, 0.0),
                    Vector2::new(1.0, 0.0),
                ],
            ),
            Face::new(
                [5, 0, 3],
                [
                    Vector2::new(0.0, 1.0),
                    Vector2::new(1.0, 0.0),
                    Vector2::new(1.0, 1.0),
                ],
            ),
        ];
        Mesh { vertices, faces }
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
        // For drawing on screen
        image: &mut Image,
        texture: &Option<Image>,
        depth_buffer: &mut Vec<f32>,
    ) {
        let mut triangles_to_raster = Vec::new();

        // Pre-calculate the transformation matrix
        let transform_mat = mat_multiply(&rotate_x(rotation.x), &rotate_y(rotation.y));
        let transform_mat = mat_multiply(&transform_mat, &rotate_z(rotation.z));
        let transform_mat = mat_multiply(
            &transform_mat,
            &translate(translation.x, translation.y, translation.z),
        );

        for face in &self.faces {
            // Transform vertices -> Rotation, Translation, Scale (Not yet implemented)
            let mut transformed_vertices = Vec::with_capacity(3);
            for v in face.vertices {
                let vertex = &self.vertices[v];

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
                    face.texture_coords,
                );

                // Clipping triangles against near plane
                let mut clipped_triangles = Vec::with_capacity(2);

                let _num_clipped_triangles = triangle_clip_plane(
                    &Vector3::forward(),
                    &Vector3::new(0.0, 0.0, NEAR),
                    &view_triangle,
                    &mut clipped_triangles,
                );

                // Project to screen: 3D -> 2D
                for clipped_triangle in clipped_triangles {
                    let mut projected_triangle = clipped_triangle.clone();

                    for i in 0..3 {
                        // Project to screen
                        let vertex = clipped_triangle.vertices[i];
                        let texture_coords = clipped_triangle.texture_coords[i];

                        let projected_vertex = mult_vec_mat(&vertex, projection_mat);

                        // Project texture coordinates (Make texture coordinates relative to z)
                        projected_triangle.texture_coords[i] =
                            vec2_div(&texture_coords, projected_vertex.w);
                        // Idk why this is needed
                        projected_triangle.texture_coords[i].w = 1.0 / projected_vertex.w;

                        // Normalize into cartesian coordinates using w component
                        let mut projected_vertex = vec_div(&projected_vertex, projected_vertex.w);

                        // Scale to screen dimensions
                        projected_vertex.x = (projected_vertex.x + 1.0) * width / 2.0;
                        projected_vertex.y = (projected_vertex.y + 1.0) * height / 2.0;

                        projected_triangle.vertices[i] = projected_vertex;
                    }

                    triangles_to_raster.push(projected_triangle);
                }
            }
        }

        // NO NEED FOR THIS SINCE WE'RE USING A DEPTH BUFFER
        //
        // Sort triangles by average depth (painter's algorithm)
        // Render triangles in order of highest depth (z-index) to lowest
        // triangles_to_raster.sort_by(|t1, t2| {
        //     let z1 = (t1.vertices[0].z + t1.vertices[1].z + t1.vertices[2].z) / 3.0;
        //     let z2 = (t2.vertices[0].z + t2.vertices[1].z + t2.vertices[2].z) / 3.0;

        //     z1.partial_cmp(&z2).unwrap()
        // });

        for triangle in triangles_to_raster {
            // Clip triangle against screen boundaries
            let mut triangle_queue = VecDeque::new();
            triangle_queue.push_back(triangle);

            // For every side
            for i in 0..4 {
                let mut temp_queue = VecDeque::new();
                while let Some(triangle_to_clip) = triangle_queue.pop_front() {
                    let mut clipped_triangles = Vec::with_capacity(2);
                    match i {
                        0 => {
                            // Top plane
                            triangle_clip_plane(
                                &Vector3::up(),
                                &Vector3::new(0.0, 0.0, 0.0),
                                &triangle_to_clip,
                                &mut clipped_triangles,
                            );
                        }
                        1 => {
                            // Bottom plane
                            triangle_clip_plane(
                                &Vector3::down(),
                                &Vector3::new(0.0, screen_height() - 0.0, 0.0),
                                &triangle_to_clip,
                                &mut clipped_triangles,
                            );
                        }
                        2 => {
                            // Left plane
                            triangle_clip_plane(
                                &Vector3::right(),
                                &Vector3::new(0.0, 0.0, 0.0),
                                &triangle_to_clip,
                                &mut clipped_triangles,
                            );
                        }
                        3 => {
                            // Right plane
                            triangle_clip_plane(
                                &Vector3::left(),
                                &Vector3::new(screen_width() - 0.0, 0.0, 0.0),
                                &triangle_to_clip,
                                &mut clipped_triangles,
                            );
                        }
                        _ => {}
                    }
                    for triangle in clipped_triangles {
                        temp_queue.push_back(triangle);
                    }
                }
                triangle_queue = temp_queue;
            }

            for clipped_triangle in triangle_queue {
                Self::draw_textured_triangle(clipped_triangle, image, texture, depth_buffer);

                // Self::draw_triangle_face(clipped_triangle);
                // Self::_draw_triangle_wireframe(clipped_triangle);
            }
        }
    }

    fn draw_textured_triangle(
        triangle: Triangle,
        image: &mut Image,
        texture: &Option<Image>,
        depth_buffer: &mut Vec<f32>,
    ) {
        use std::mem::swap;
        // Order vertices and texture coordinates by y-coordinate of vertex
        let mut y1 = triangle.vertices[0].y as i32;
        let mut y2 = triangle.vertices[1].y as i32;
        let mut y3 = triangle.vertices[2].y as i32;
        let mut x1 = triangle.vertices[0].x as i32;
        let mut x2 = triangle.vertices[1].x as i32;
        let mut x3 = triangle.vertices[2].x as i32;

        let mut u1 = triangle.texture_coords[0].u;
        let mut u2 = triangle.texture_coords[1].u;
        let mut u3 = triangle.texture_coords[2].u;
        let mut v1 = triangle.texture_coords[0].v;
        let mut v2 = triangle.texture_coords[1].v;
        let mut v3 = triangle.texture_coords[2].v;
        // NOTE THAT THE W HERE ARE FROM TEXTURE COORDINATES
        let mut w1 = triangle.texture_coords[0].w;
        let mut w2 = triangle.texture_coords[1].w;
        let mut w3 = triangle.texture_coords[2].w;

        if y2 < y1 {
            swap(&mut y1, &mut y2);
            swap(&mut x1, &mut x2);
            swap(&mut u1, &mut u2);
            swap(&mut v1, &mut v2);
            swap(&mut w1, &mut w2);
        }

        if y3 < y1 {
            swap(&mut y1, &mut y3);
            swap(&mut x1, &mut x3);
            swap(&mut u1, &mut u3);
            swap(&mut v1, &mut v3);
            swap(&mut w1, &mut w3);
        }

        if y3 < y2 {
            swap(&mut y2, &mut y3);
            swap(&mut x2, &mut x3);
            swap(&mut u2, &mut u3);
            swap(&mut v2, &mut v3);
            swap(&mut w2, &mut w3);
        }

        // These are integers as the number represents pixels, which cannot be floats
        let dy1 = (y2 - y1) as i32;
        let dx1 = (x2 - x1) as i32;

        let dv1 = v2 - v1;
        let du1 = u2 - u1;
        let dw1 = w2 - w1;

        // These are integers as the number represents pixels, which cannot be floats
        let dy2 = (y3 - y1) as i32;
        let dx2 = (x3 - x1) as i32;

        let dv2 = v3 - v1;
        let du2 = u3 - u1;
        let dw2 = w3 - w1;

        // Change in x for a unit change in y for A and B sides
        let mut dax_step = 0.0;
        let mut dbx_step = 0.0;

        // Same as above but for u and vj
        let mut du1_step = 0.0;
        let mut du2_step = 0.0;

        let mut dv1_step = 0.0;
        let mut dv2_step = 0.0;

        let mut dw1_step = 0.0;
        let mut dw2_step = 0.0;

        if dy1 != 0 {
            dax_step = dx1 as f32 / dy1.abs() as f32;
            du1_step = du1 as f32 / dy1.abs() as f32;
            dv1_step = dv1 as f32 / dy1.abs() as f32;
            dw1_step = dw1 as f32 / dy1.abs() as f32;
        }
        if dy2 != 0 {
            dbx_step = dx2 as f32 / dy2.abs() as f32;
            du2_step = du2 as f32 / dy2.abs() as f32;
            dv2_step = dv2 as f32 / dy2.abs() as f32;
            dw2_step = dw2 as f32 / dy2.abs() as f32;
        }

        // First half of the triangle if it is not flat
        if dy1 != 0 {
            // For every scanline between y1 and y2
            for i in y1 as i32..y2 as i32 {
                // Ax and Bx are the starting and ending x values in a scanline repectively
                let mut ax = (x1 as f32 + dax_step * (i - y1) as f32) as i32;
                let mut bx = (x1 as f32 + dbx_step * (i - y1) as f32) as i32;

                // Same but for starting texture coordinates
                let mut tex_su = u1 + du1_step * (i - y1) as f32;
                let mut tex_sv = v1 + dv1_step * (i - y1) as f32;
                let mut tex_sw = w1 + dw1_step * (i - y1) as f32;
                // Same but for ending texture coordinates
                let mut tex_eu = u1 + du2_step * (i - y1) as f32;
                let mut tex_ev = v1 + dv2_step * (i - y1) as f32;
                let mut tex_ew = w1 + dw2_step * (i - y1) as f32;

                // Ensure that ax < bx => Drawing from left to right
                if ax > bx {
                    swap(&mut ax, &mut bx);
                    swap(&mut tex_su, &mut tex_eu);
                    swap(&mut tex_sv, &mut tex_ev);
                    swap(&mut tex_sw, &mut tex_ew);
                }

                // t represents the normalized position between ax and bx => Where we are in the scanline
                let t_step = 1.0 / (bx - ax) as f32;
                let mut t = 0.0;

                for j in ax..=bx {
                    let tex_u = (1.0 - t) * tex_su + t * tex_eu;
                    let tex_v = (1.0 - t) * tex_sv + t * tex_ev;
                    let tex_w = (1.0 - t) * tex_sw + t * tex_ew;

                    let color = if let Some(texture) = texture {
                        let tex_x = ((tex_u / tex_w) * texture.width() as f32) as u32;
                        let tex_y = ((tex_v / tex_w) * texture.height() as f32) as u32;

                        let tex_x = tex_x.clamp(0, texture.width().saturating_sub(1) as u32);
                        let tex_y = tex_y.clamp(0, texture.height().saturating_sub(1) as u32);

                        let color = texture.get_pixel(tex_x, tex_y);

                        color
                    } else {
                        let color_value = triangle.intensity as u8;
                        let color = Color::from_rgba(color_value, color_value, color_value, 255);
                        color
                    };

                    if j < image.width() as i32 && i < image.height() as i32 {
                        // Update depth buffer
                        let pixel_depth =
                            depth_buffer[j as usize + i as usize * image.width() as usize];

                        if tex_w < pixel_depth {
                            image.set_pixel(j as u32, i as u32, color);
                            depth_buffer[j as usize + i as usize * image.width() as usize] = tex_w;
                        }
                    }

                    t += t_step;
                }
            }
        }

        // Resetting values for the second half of the triangle
        let dy1 = y3 - y2;
        let dx1 = x3 - x2;

        let dv1 = v3 - v2;
        let du1 = u3 - u2;
        let dw1 = w3 - w2;

        du1_step = 0.0;
        dv1_step = 0.0;
        dw1_step = 0.0;

        if dy1 != 0 {
            dax_step = dx1 as f32 / dy1.abs() as f32;
            du1_step = du1 as f32 / dy1.abs() as f32;
            dv1_step = dv1 as f32 / dy1.abs() as f32;
            dw1_step = dw1 as f32 / dy1.abs() as f32;
        }
        if dy2 != 0 {
            dbx_step = dx2 as f32 / dy2.abs() as f32;
        }

        if dy1 != 0 {
            for i in y2 as i32..y3 as i32 {
                // Ax and Bx are the starting and ending x values in a scanline repectively
                let mut ax = (x2 as f32 + dax_step * (i - y2) as f32) as i32;
                let mut bx = (x1 as f32 + dbx_step * (i - y1) as f32) as i32;

                // Same but for starting texture coordinates
                let mut tex_su = u2 + du1_step * (i - y2) as f32;
                let mut tex_sv = v2 + dv1_step * (i - y2) as f32;
                let mut tex_sw = w2 + dw1_step * (i - y2) as f32;

                // Same but for ending texture coordinates
                let mut tex_eu = u1 + du2_step * (i - y1) as f32;
                let mut tex_ev = v1 + dv2_step * (i - y1) as f32;
                let mut tex_ew = w1 + dw2_step * (i - y1) as f32;

                // Ensure that ax < bx => Drawing from left to right
                if ax > bx {
                    swap(&mut ax, &mut bx);
                    swap(&mut tex_su, &mut tex_eu);
                    swap(&mut tex_sv, &mut tex_ev);
                    swap(&mut tex_sw, &mut tex_ew);
                }

                // t represents the normalized position between ax and bx => Where we are in the scanline
                let t_step = 1.0 / (bx - ax) as f32;
                let mut t = 0.0;

                for j in ax..=bx {
                    let tex_u = (1.0 - t) * tex_su + t * tex_eu;
                    let tex_v = (1.0 - t) * tex_sv + t * tex_ev;
                    let tex_w = (1.0 - t) * tex_sw + t * tex_ew;

                    let color = if let Some(texture) = texture {
                        let tex_x = ((tex_u / tex_w) * texture.width() as f32) as u32;
                        let tex_y = ((tex_v / tex_w) * texture.height() as f32) as u32;

                        let tex_x = tex_x.clamp(0, texture.width().saturating_sub(1) as u32);
                        let tex_y = tex_y.clamp(0, texture.height().saturating_sub(1) as u32);

                        let color = texture.get_pixel(tex_x, tex_y);
                        color
                    } else {
                        let color_value = triangle.intensity as u8;
                        let color = Color::from_rgba(color_value, color_value, color_value, 255);
                        color
                    };

                    if j < image.width() as i32 && i < image.height() as i32 {
                        // Update depth buffer
                        let pixel_depth =
                            depth_buffer[j as usize + i as usize * image.width() as usize];

                        if tex_w < pixel_depth {
                            image.set_pixel(j as u32, i as u32, color);
                            depth_buffer[j as usize + i as usize * image.width() as usize] = tex_w;
                        }
                    }

                    t += t_step;
                }
            }
        }
    }

    fn _draw_triangle_face(triangle: Triangle) {
        let color_value = triangle.intensity.clamp(50.0, 255.0) as u8;
        // Draw face
        draw_triangle(
            Vec2::new(triangle.vertices[0].x, triangle.vertices[0].y),
            Vec2::new(triangle.vertices[1].x, triangle.vertices[1].y),
            Vec2::new(triangle.vertices[2].x, triangle.vertices[2].y),
            Color::from_rgba(color_value, color_value, color_value, 255),
        );
    }

    fn _draw_triangle_wireframe(triangle: Triangle) {
        let color = Color::from_rgba(255, 255, 255, 255);
        // Draw wireframe
        draw_line(
            triangle.vertices[0].x,
            triangle.vertices[0].y,
            triangle.vertices[1].x,
            triangle.vertices[1].y,
            1.0,
            color,
        );
        draw_line(
            triangle.vertices[1].x,
            triangle.vertices[1].y,
            triangle.vertices[2].x,
            triangle.vertices[2].y,
            1.0,
            color,
        );
        draw_line(
            triangle.vertices[2].x,
            triangle.vertices[2].y,
            triangle.vertices[0].x,
            triangle.vertices[0].y,
            1.0,
            color,
        );
    }
}
