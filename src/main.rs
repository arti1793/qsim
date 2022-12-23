use bevy::{
    app::App,
    prelude::{BuildChildren, Camera3dBundle, Commands, Query, Transform, Vec3, With},
    transform::TransformBundle,
    ui::PositionType,
    DefaultPlugins,
};
use bevy_rapier3d::{
    prelude::{
        AdditionalMassProperties, Collider, ExternalForce, MassProperties, NoUserData,
        RapierPhysicsPlugin, Restitution, RigidBody,
    },
    render::RapierDebugRenderPlugin,
};

struct Config {
    l: f32,   // м радиус рамы
    t: f32,   // м толщина рамы
    rs: f32,  //м радиус центральной "сферы"
    ms: f32,  // г масса центральной сферы
    ml: f32,  // г масса рамы
    mm: f32,  // г масса двигателя
    mmm: f32, // г радиус сферы двигателя
}
const CONFIG: Config = Config {
    l: 0.55,   // м радиус рамы
    t: 0.02,   // м толщина рамы
    ml: 0.25,  // масса одной рамы
    rs: 0.070, //м радиус центральной "сферы"
    ms: 200.0, // г масса центральной сферы
    mm: 0.56,  // масса двигателя
    mmm: 0.07, // радиус сферы двигателя
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup_graphics)
        .add_startup_system(setup_physics)
        .add_system(print_ball_altitude)
        .run();
}

fn setup_graphics(mut commands: Commands) {
    // Add a camera so we can see the debug-render.
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-3.0, 3.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}

fn setup_physics(mut commands: Commands) {
    /* Create the ground. */
    commands
        .spawn(TransformBundle::from(Transform::from_xyz(0.0, -2.0, 0.0)))
        .insert(Collider::cuboid(100.0, 0.1, 100.0));

    commands
        .spawn(RigidBody::Dynamic)
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 10.0, 0.0)))
        .insert(AdditionalMassProperties::Mass(0.005))
        // .insert(ExternalForce {
        // //     // force: Vec3 {
        // //     //     x: 0.0,
        // //     //     y: 100000.0,
        // //     //     z: 0.0,
        // //     // },
        // //     torque: Vec3 {
        // //         x: 1000000.0,
        // //         y: 0.0,
        // //         z: 0.0,
        // //     },
        //     ..Default::default()
        // })
        .insert(Restitution::coefficient(0.7))
        .with_children(|children| {
            children
                .spawn(Collider::cuboid(CONFIG.l, CONFIG.t, CONFIG.t))
                .insert(AdditionalMassProperties::Mass(CONFIG.ml))
                .insert(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)));
            children
                .spawn(Collider::cuboid(CONFIG.t, CONFIG.t, CONFIG.l))
                .insert(AdditionalMassProperties::Mass(CONFIG.ml))
                .insert(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)));

            children
                .spawn(Collider::ball(CONFIG.rs))
                .insert(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)))
                .insert(AdditionalMassProperties::Mass(CONFIG.ms));

            children
                .spawn(Collider::ball(CONFIG.mmm))
                .insert(TransformBundle::from(Transform::from_xyz(
                    CONFIG.l, 0.0, 0.0,
                )))
                .insert(AdditionalMassProperties::Mass(CONFIG.mm));
            // .insert(ExternalForce {
            // force: Vec3 {
            //     x: 1000000.0,
            //     y: 0.0,
            //     z: 0.0,
            // },
            // torque: Vec3 {
            //     x: 0.0,
            //     y: 1.0,
            //     z: 0.0,
            // },
            // ..Default::default()
            // });
            children
                .spawn(Collider::ball(CONFIG.mmm))
                .insert(TransformBundle::from(Transform::from_xyz(
                    -CONFIG.l, 0.0, 0.0,
                )))
                .insert(AdditionalMassProperties::Mass(CONFIG.mm));
            // .insert(ExternalForce {
            // force: Vec3 {
            //     x: 0.0,
            //     y: 100000.0,
            //     z: 0.0,
            // },
            // torque: Vec3 {
            //     x: 0.0,
            //     y: 1.0,
            //     z: 0.0,
            // },
            // ..Default::default()
            // });
            children
                .spawn(Collider::ball(CONFIG.mmm))
                .insert(TransformBundle::from(Transform::from_xyz(
                    0.0, 0.0, CONFIG.l,
                )))
                .insert(AdditionalMassProperties::Mass(CONFIG.mm));
            // .insert(ExternalForce {
            // force: Vec3 {
            //     x: 0.0,
            //     y: 100000.0,
            //     z: 0.0,
            // },
            // torque: Vec3 {
            //     x: 0.0,
            //     y: 1.0,
            //     z: 0.0,
            // },
            // ..Default::default()
            // });
            children
                .spawn(Collider::ball(CONFIG.mmm))
                .insert(TransformBundle::from(Transform::from_xyz(
                    0.0, 0.0, -CONFIG.l,
                )))
                .insert(AdditionalMassProperties::Mass(CONFIG.mm));
            // .insert(ExternalForce {
            //     force: Vec3 {
            //         x: 0.0,
            //         y: 100000.0,
            //         z: 0.0,
            //     },
            //     torque: Vec3 {
            //         x: 0.0,
            //         y: 0.0,
            //         z: 0.0,
            //     },
            //     ..Default::default()
            // });
        });
}

fn print_ball_altitude(positions: Query<&Transform, With<RigidBody>>) {
    for transform in positions.iter() {
        println!(
            "Ball altitude: {}, {}",
            transform.translation.y, transform.rotation.y
        );
    }
}
