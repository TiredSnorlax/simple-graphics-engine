use graphics_engine::{Camera, Mesh, Object, Vector3, matrix};
use macroquad::{
    color::{BLACK, WHITE},
    texture::{Image, Texture2D, draw_texture, load_image},
    time::draw_fps,
    window::{next_frame, screen_height, screen_width},
};

#[macroquad::main("BasicShapes")]
async fn main() {
    let mesh = Mesh::load_from_obj("assets/map/Artisans Hub.obj", true).unwrap();
    // let mesh = Mesh::cube();
    let object = Object {
        mesh,
        position: Vector3::new(0.0, 0.0, 5.0),
        rotation: Vector3::default(),
        texture: Some(load_image("assets/map/High.png").await.unwrap()),
        // texture: Some(load_image("assets/mario.png").await.unwrap()),
        // texture: None,
    };

    let mut objs = vec![object];

    let mut camera = Camera::new();

    let light_direction = Vector3::new(0.0, 0.0, -1.0).normalize();

    let projection_matrix =
        matrix::projection_matrix(screen_width() / screen_height(), 90.0, 0.1, 100.0);

    let mut image = Image::gen_image_color(screen_width() as u16, screen_height() as u16, BLACK);
    let img_texture = Texture2D::from_image(&image);

    loop {
        // Reset depth buffer for next drawing
        image.bytes.fill(0); // Clear the image to black efficiently
        let mut depth_buffer = vec![0.0; (screen_width() * screen_height()) as usize];

        camera.handle_user_input();
        tick(&mut objs);
        draw(
            &mut objs,
            &camera,
            &light_direction,
            &projection_matrix,
            &mut image,
            &mut depth_buffer,
        );

        img_texture.update(&image);
        draw_texture(&img_texture, 0., 0., WHITE);

        draw_fps();

        next_frame().await
    }
}

fn tick(objects: &mut Vec<Object>) {
    for _object in objects {
        // object.rotation.x += 0.005;
        // object.rotation.y += 0.01;
        // object.rotation.z += 0.01;
    }
}

fn draw(
    objects: &mut Vec<Object>,
    camera: &Camera,
    light_direction: &Vector3,
    projection_mat: &matrix::Mat4x4,
    image: &mut Image,
    depth_buffer: &mut Vec<f32>,
) {
    let view_mat = camera.return_view_mat();
    for object in objects {
        object.draw(
            screen_width(),
            screen_height(),
            camera,
            light_direction,
            projection_mat,
            &view_mat,
            image,
            depth_buffer,
        );
    }
}
