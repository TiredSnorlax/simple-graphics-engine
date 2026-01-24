use crate::mesh::{Triangle, Vertex};

#[derive(Debug, Clone, Copy, Default)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vector3 { x, y, z, w: 1.0 }
    }

    pub fn normalize(&self) -> Self {
        let length = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        Vector3::new(self.x / length, self.y / length, self.z / length)
    }

    pub fn up() -> Self {
        Vector3::new(0.0, 1.0, 0.0)
    }

    pub fn down() -> Self {
        Vector3::new(0.0, -1.0, 0.0)
    }

    pub fn left() -> Self {
        Vector3::new(-1.0, 0.0, 0.0)
    }

    pub fn right() -> Self {
        Vector3::new(1.0, 0.0, 0.0)
    }

    pub fn forward() -> Self {
        Vector3::new(0.0, 0.0, 1.0)
    }
}

pub fn vec_add(v1: &Vector3, v2: &Vector3) -> Vector3 {
    Vector3::new(v1.x + v2.x, v1.y + v2.y, v1.z + v2.z)
}

pub fn vec_sub(v1: &Vector3, v2: &Vector3) -> Vector3 {
    Vector3::new(v1.x - v2.x, v1.y - v2.y, v1.z - v2.z)
}

pub fn vec_add_a(v1: &Vector3, a: f32) -> Vector3 {
    Vector3::new(v1.x + a, v1.y + a, v1.z + a)
}

pub fn vec_sub_a(v1: &Vector3, a: f32) -> Vector3 {
    Vector3::new(v1.x - a, v1.y - a, v1.z - a)
}

pub fn vec_div(v1: &Vector3, divisor: f32) -> Vector3 {
    Vector3::new(v1.x / divisor, v1.y / divisor, v1.z / divisor)
}

pub fn vec_mul(v1: &Vector3, mult: f32) -> Vector3 {
    Vector3::new(v1.x * mult, v1.y * mult, v1.z * mult)
}

pub fn cross_product(vec1: &Vector3, vec2: &Vector3) -> Vector3 {
    Vector3::new(
        vec1.y * vec2.z - vec1.z * vec2.y,
        vec1.z * vec2.x - vec1.x * vec2.z,
        vec1.x * vec2.y - vec1.y * vec2.x,
    )
}

pub fn dot_product(vec1: &Vector3, vec2: &Vector3) -> f32 {
    vec1.x * vec2.x + vec1.y * vec2.y + vec1.z * vec2.z
}

pub type Mat4x4 = [[f32; 4]; 4];

pub fn mat_multiply(mat1: &Mat4x4, mat2: &Mat4x4) -> Mat4x4 {
    let mut result = [[0.0; 4]; 4];

    for i in 0..4 {
        for j in 0..4 {
            for k in 0..4 {
                result[i][j] += mat1[i][k] * mat2[k][j];
            }
        }
    }

    result
}

pub fn projection_matrix(aspect_ratio: f32, fov: f32, near: f32, far: f32) -> Mat4x4 {
    let fov = fov / 180.0 * std::f32::consts::PI;
    let f = 1.0 / (fov / 2.0).tan();

    let mut mat = [[0f32; 4]; 4];

    mat[0][0] = f / aspect_ratio;
    mat[1][1] = f;
    mat[2][2] = -(far + near) / (far - near);
    mat[2][3] = -1.0;
    mat[3][2] = -(2.0 * far * near) / (far - near);

    mat
}

pub fn rotate_x(a: f32) -> Mat4x4 {
    let mut mat = [[0.0; 4]; 4];
    mat[0][0] = 1.0;
    mat[1][1] = a.cos();
    mat[1][2] = -a.sin();
    mat[2][1] = a.sin();
    mat[2][2] = a.cos();
    mat[3][3] = 1.0;
    mat
}

pub fn rotate_y(a: f32) -> Mat4x4 {
    let mut mat = [[0.0; 4]; 4];
    mat[0][0] = a.cos();
    mat[0][2] = a.sin();
    mat[1][1] = 1.0;
    mat[2][0] = -a.sin();
    mat[2][2] = a.cos();
    mat[3][3] = 1.0;
    mat
}

pub fn rotate_z(a: f32) -> Mat4x4 {
    let mut mat = [[0.0; 4]; 4];
    mat[0][0] = a.cos();
    mat[0][1] = -a.sin();
    mat[1][0] = a.sin();
    mat[1][1] = a.cos();
    mat[2][2] = 1.0;
    mat[3][3] = 1.0;
    mat
}

pub fn translate(x: f32, y: f32, z: f32) -> Mat4x4 {
    let mut mat = [[0.0; 4]; 4];
    mat[0][0] = 1.0;
    mat[1][1] = 1.0;
    mat[2][2] = 1.0;
    mat[3][3] = 1.0;
    mat[3][0] = x;
    mat[3][1] = y;
    mat[3][2] = z;
    mat
}

pub fn point_at_mat(pos: &Vector3, target: &Vector3, up: &Vector3) -> Mat4x4 {
    // Getting the direction vectors of the camera
    let forward = (vec_sub(target, pos)).normalize();

    let a = vec_mul(&forward, dot_product(up, &forward));
    let new_up = vec_sub(up, &a).normalize();

    let new_right = cross_product(&new_up, &forward);

    // Translation of the camera with respect to the new coordinate/direction system
    let translation_x = dot_product(pos, &new_right);
    let translation_y = dot_product(pos, &new_up);
    let translation_z = dot_product(pos, &forward);

    let mut mat = [[0.0; 4]; 4];
    mat[0][0] = new_right.x;
    mat[0][1] = new_right.y;
    mat[0][2] = new_right.z;
    mat[0][3] = translation_x;

    mat[1][0] = new_up.x;
    mat[1][1] = new_up.y;
    mat[1][2] = new_up.z;
    mat[1][3] = translation_y;

    mat[2][0] = forward.x;
    mat[2][1] = forward.y;
    mat[2][2] = forward.z;
    mat[2][3] = translation_z;

    mat[3][0] = pos.x;
    mat[3][1] = pos.y;
    mat[3][2] = pos.z;
    mat[3][3] = 1.0;

    mat
}

pub fn quick_inverse_mat(mat: &Mat4x4) -> Mat4x4 {
    let mut inv = [[0.0; 4]; 4];

    inv[0][0] = mat[0][0];
    inv[0][1] = mat[1][0];
    inv[0][2] = mat[2][0];
    inv[0][3] = 0.0;

    inv[1][0] = mat[0][1];
    inv[1][1] = mat[1][1];
    inv[1][2] = mat[2][1];
    inv[1][3] = 0.0;

    inv[2][0] = mat[0][2];
    inv[2][1] = mat[1][2];
    inv[2][2] = mat[2][2];
    inv[2][3] = 0.0;

    inv[3][0] = -(mat[0][3]);
    inv[3][1] = -(mat[1][3]);
    inv[3][2] = -(mat[2][3]);
    inv[3][3] = 1.0;

    inv
}

pub fn mult_vec_mat(vec: &Vector3, mat: &Mat4x4) -> Vector3 {
    let mut result = Vector3::new(0.0, 0.0, 0.0);

    result.x = vec.x * mat[0][0] + vec.y * mat[1][0] + vec.z * mat[2][0] + vec.w * mat[3][0];
    result.y = vec.x * mat[0][1] + vec.y * mat[1][1] + vec.z * mat[2][1] + vec.w * mat[3][1];
    result.z = vec.x * mat[0][2] + vec.y * mat[1][2] + vec.z * mat[2][2] + vec.w * mat[3][2];
    result.w = vec.x * mat[0][3] + vec.y * mat[1][3] + vec.z * mat[2][3] + vec.w * mat[3][3];

    result
}

pub fn line_plane_intersection(
    plane_normal: &Vector3,
    plane_point: &Vector3,
    line_start: &Vector3,
    line_end: &Vector3,
) -> Vector3 {
    let plane_normal = plane_normal.normalize();
    let d = dot_product(&plane_normal, plane_point);
    let line_direction = vec_sub(line_end, line_start);
    let t =
        (d - dot_product(line_start, &plane_normal)) / dot_product(&plane_normal, &line_direction);
    let intersection = vec_add(&line_start, &vec_mul(&line_direction, t));
    intersection
}

// This is signed -> Positive distance means the point is in front of the plane (relative to normal)
fn dist_point_plane(point: &Vertex, plane_normal: &Vector3, plane_point: &Vector3) -> f32 {
    return (plane_normal.x * point.x + plane_normal.y * point.y + plane_normal.z * point.z)
        - dot_product(plane_normal, plane_point);
}

pub fn triangle_clip_plane(
    plane_normal: &Vector3,
    plane_point: &Vector3,
    triangle: &Triangle,
    out_triangles: &mut Vec<Triangle>,
) -> usize {
    let plane_normal = plane_normal.normalize();

    let mut inside_points: Vec<&Vertex> = Vec::with_capacity(3);
    let mut outside_points: Vec<&Vertex> = Vec::with_capacity(3);

    for vertex in &triangle.vertices {
        let distance = dist_point_plane(vertex, &plane_normal, &plane_point);
        if distance >= 0.0 {
            inside_points.push(vertex);
        } else {
            outside_points.push(vertex);
        }
    }

    let inside_count = inside_points.len();
    let outside_count = outside_points.len();
    if inside_count == 0 {
        // All points are outside the plane -> Clip whole triangle
        return 0;
    }

    if inside_count == 3 {
        // All points are inside the plane -> No clipping needed
        out_triangles.push(*triangle);

        return 1;
    }

    if inside_count == 1 && outside_count == 2 {
        // One point is inside, two points are outside -> Clip triangle into one triangles
        let inside_point = inside_points[0];
        let outside_point1 = outside_points[0];
        let outside_point2 = outside_points[1];

        let intersection1 =
            line_plane_intersection(&plane_normal, &plane_point, inside_point, outside_point1);
        let intersection2 =
            line_plane_intersection(&plane_normal, &plane_point, inside_point, outside_point2);

        out_triangles.push(Triangle {
            vertices: [*inside_point, intersection1, intersection2],
            intensity: triangle.intensity,
        });

        return 1;
    }

    if inside_count == 2 && outside_count == 1 {
        // Two points are inside, one point is outside -> Clip triangle into two triangles
        let inside_point1 = inside_points[0];
        let inside_point2 = inside_points[1];
        let outside_point = outside_points[0];

        let intersection1 =
            line_plane_intersection(&plane_normal, &plane_point, inside_point1, outside_point);
        let intersection2 =
            line_plane_intersection(&plane_normal, &plane_point, inside_point2, outside_point);

        let out_triangle1 = Triangle {
            vertices: [*inside_point1, *inside_point2, intersection1],
            intensity: triangle.intensity,
        };

        let out_triangle2 = Triangle {
            vertices: [*inside_point2, intersection2, intersection1],
            intensity: triangle.intensity,
        };

        out_triangles.push(out_triangle1);
        out_triangles.push(out_triangle2);

        return 2;
    }

    return 0;
}
