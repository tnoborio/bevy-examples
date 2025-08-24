use bevy::prelude::*;

#[derive(Component)]
struct AnimationIndices {
    order: Vec<usize>,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(ImagePlugin::default_nearest())
        )
        .add_systems(Startup, setup)
        .add_systems(Update, advance_seq)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let sheet_size = UVec2::new(508, 288);
    let mut layout = TextureAtlasLayout::new_empty(sheet_size);

    let order = vec![
        layout.add_texture(URect::from_corners(UVec2::new(0, 0), UVec2::new(72 + 0, 97 + 0))),
        layout.add_texture(URect::from_corners(UVec2::new(73, 0), UVec2::new(72 + 73, 97 + 0))),
        layout.add_texture(URect::from_corners(UVec2::new(146, 0), UVec2::new(72 + 146, 97 + 0))),
        layout.add_texture(URect::from_corners(UVec2::new(0, 98), UVec2::new(72 + 0, 97 + 98))),
        layout.add_texture(URect::from_corners(UVec2::new(73, 98), UVec2::new(72 + 73, 97 + 98))),
        layout.add_texture(URect::from_corners(UVec2::new(146, 98), UVec2::new(72 + 146, 97 + 98))),
        layout.add_texture(URect::from_corners(UVec2::new(219, 0), UVec2::new(72 + 219, 97 + 0))),
        layout.add_texture(URect::from_corners(UVec2::new(292, 0), UVec2::new(72 + 292, 97 + 0))),
        layout.add_texture(URect::from_corners(UVec2::new(219, 98), UVec2::new(72 + 219, 97 + 98))),
        layout.add_texture(URect::from_corners(UVec2::new(365, 0), UVec2::new(72 + 365, 97 + 0))),
        layout.add_texture(URect::from_corners(UVec2::new(292, 98), UVec2::new(72 + 292, 97 + 98))),
    ];

    let layout_handle = layouts.add(layout);

    let texture = asset_server.load("platformer-art-complete-pack/Base pack/Player/p3_spritesheet.png");

    commands.spawn(Camera2d);

    commands.spawn((
        Sprite::from_atlas_image(
            texture,
            TextureAtlas { layout: layout_handle, index: order[0] },
        ),
        Transform::from_scale(Vec3::splat(1.25)),
        AnimationIndices { order },
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    ));
}

fn advance_seq(
    time: Res<Time>,
    mut q: Query<(&mut Sprite, &mut AnimationTimer, &mut AnimationIndices)>,
) {
    for (mut sprite, mut timer, indices) in &mut q {
        timer.tick(time.delta());

        if timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = if atlas.index == *indices.order.last().unwrap() {
                    *indices.order.first().unwrap()
                } else {
                    atlas.index + 1
                };
            }
        }
    }
}