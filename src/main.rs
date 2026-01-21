use graphics_engine::{Mesh, Object, Vector3};
use macroquad::{
    time::draw_fps,
    window::{next_frame, screen_height, screen_width},
};

#[macroquad::main("BasicShapes")]
async fn main() {
    // let cube = Object::cube(Vector3 {
    //     x: 0.0,
    //     y: 0.0,
    //     z: 3.0,
    // });
    let spaceship_mesh = Mesh::load_from_obj("assets/spaceship.obj").unwrap();
    let spaceship = Object {
        mesh: spaceship_mesh,
        position: Vector3 {
            x: 0.0,
            y: 0.0,
            z: 10.0,
        },
        rotation: Vector3::default(),
    };
    let mut objs = vec![spaceship];

    let camera = Vector3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    let light_direction = Vector3::new(0.0, 0.0, -1.0).normalize();

    loop {
        tick(&mut objs);
        draw(&mut objs, &camera, &light_direction);

        draw_fps();
        next_frame().await
    }
}

fn tick(objects: &mut Vec<Object>) {
    for object in objects {
        object.rotation.x += 0.01;
        object.rotation.y += 0.02;
        object.rotation.z += 0.03;
    }
}
fn draw(objects: &mut Vec<Object>, camera: &Vector3, light_direction: &Vector3) {
    for object in objects {
        object.draw(screen_width(), screen_height(), camera, light_direction);
    }
}
