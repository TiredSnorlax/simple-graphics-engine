use graphics_engine::{Mesh, Object, Vector3, matrix};
use macroquad::{
    time::draw_fps,
    window::{next_frame, screen_height, screen_width},
};

#[macroquad::main("BasicShapes")]
async fn main() {
    // let spaceship_mesh = Mesh::load_from_obj("assets/spaceship.obj").unwrap();
    // let spaceship = Object {
    //     mesh: spaceship_mesh,
    //     position: Vector3::new(0.0, 0.0, 10.0),
    //     rotation: Vector3::default(),
    // };
    let axis_mesh = Mesh::load_from_obj("assets/axis.obj").unwrap();
    let axis = Object {
        mesh: axis_mesh,
        position: Vector3::new(0.0, 0.0, 5.0),
        rotation: Vector3::default(),
    };

    let mut objs = vec![axis];

    let camera = Vector3::new(0.0, 0.0, 0.0);

    let light_direction = Vector3::new(0.0, 0.0, -1.0).normalize();

    let projection_matrix =
        matrix::projection_matrix(screen_width() / screen_height(), 90.0, 0.1, 100.0);

    loop {
        tick(&mut objs);
        draw(&mut objs, &camera, &light_direction, &projection_matrix);

        draw_fps();
        next_frame().await
    }
}

fn tick(objects: &mut Vec<Object>) {
    for object in objects {
        // object.rotation.x += 0.005;
        // object.rotation.y += 0.01;
        // object.rotation.z += 0.01;
    }
}
fn draw(
    objects: &mut Vec<Object>,
    camera: &Vector3,
    light_direction: &Vector3,
    projection_mat: &matrix::Mat4x4,
) {
    for object in objects {
        object.draw(
            screen_width(),
            screen_height(),
            camera,
            light_direction,
            projection_mat,
        );
    }
}
