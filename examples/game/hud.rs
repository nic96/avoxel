use bevy::{prelude::*, ui::FocusPolicy};

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system());
    }
}

struct HudItem;
pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(UiCameraBundle::default());

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            material: materials.add(Color::NONE.into()),
            visible: Visible {
                is_visible: true,
                is_transparent: true,
            },
            ..Default::default()
        })
        .with_children(|parent| {
            // bevy logo (image)
            parent
                .spawn_bundle(ImageBundle {
                    style: Style {
                        size: Size::new(Val::Px(32.0), Val::Auto),
                        ..Default::default()
                    },
                    material: materials.add(asset_server.load("textures/cross_hairs.png").into()),
                    visible: Visible {
                        is_visible: true,
                        is_transparent: false,
                    },
                    ..Default::default()
                })
                .insert(HudItem);
        })
        .insert(FocusPolicy::Pass)
        .insert(HudItem);
}
