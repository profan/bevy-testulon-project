use bevy::prelude::*;
use rand::prelude::*;

const MOVE_SPEED: f32 = 32.0f32;
const NUM_PARTICLES: i32 = 100;
const REPEAT_RATE: f32 = 2.0f32;

struct Particle {
    id: i32,
    velocity: Vec3,
    speed: f32
}

fn create_the_particles(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {

    // textures
    let texture_asset = asset_server.load("assets/branding/icon.png").unwrap();
    let texture_material = materials.add(texture_asset.into());
    let color_material = materials.add(Color::rgba(1.0, 0.0, 0.0, 1.0).into());

    // create the camera
    commands.spawn(Camera2dComponents::default());

    // put a bevy logo up in there
    commands.spawn(SpriteComponents {
        material: texture_material,
        ..Default::default()
    });

    // spawn some particles
    for i in 0..NUM_PARTICLES {
        let v_x = ((rand::random::<f32>() - 0.5) * 2.0) * MOVE_SPEED;
        let v_y = ((rand::random::<f32>() - 0.5) * 2.0) * MOVE_SPEED;
        commands
            .spawn(SpriteComponents {
                material: color_material,
                sprite: Sprite::new(Vec2::new(64.0, 64.0)),
                ..Default::default()
            })
            .with(Particle {
                id: i,
                speed: 1.0f32,
                velocity: Vec3::new(v_x, v_y, 0.0)
            });
    }

}

struct ParticleTimer(Timer);

fn move_the_particles(
    game_time: Res<Time>,
    window: Res<Windows>,
    mut timer: ResMut<ParticleTimer>,
    mut query: Query<(&mut Particle, &mut Transform)>
) {

    let dt = game_time.delta_seconds;
    timer.0.tick(game_time.delta_seconds);

    let primary_window = window.get_primary().unwrap();
    let w = (primary_window.width / 2) as f32;
    let h = (primary_window.height / 2) as f32;

   if timer.0.finished { // if our particle timer hits, recalculate velocities
        for (mut p, mut t) in &mut query.iter() {
            let v_x = ((rand::random::<f32>() - 0.5) * 2.0) * MOVE_SPEED;
            let v_y = ((rand::random::<f32>() - 0.5) * 2.0) * MOVE_SPEED;
            p.velocity.set_x(v_x);
            p.velocity.set_y(v_y);
        }
   }
   else
   {
        for (mut p, mut t) in &mut query.iter() {
            if t.translation().x() < -w || t.translation().x() > w {
                *p.velocity.x_mut() *= -1.0;
            }
            if t.translation().y() < -h || t.translation().y() > h {
                *p.velocity.y_mut() *= -1.0;
            }
            t.translate(p.velocity * dt);
        }
   }

}

fn main() {

    App::build()
        .add_default_plugins()
        .add_resource(ClearColor(Color::rgb(0.2, 0.2, 0.8)))
        .add_resource(ParticleTimer(Timer::from_seconds(REPEAT_RATE, true)))
        .add_startup_system(create_the_particles.system())
        .add_system(move_the_particles.system())
        .run();

}
