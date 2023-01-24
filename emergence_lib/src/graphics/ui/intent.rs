//! Displays available intent and selected option

use super::{
    bevy_widget_preview::{StatusBarInner, StatusBarWidget},
    LeftPanel,
};
use bevy::prelude::*;

fn intent_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    ui_panel: Query<Entity, With<LeftPanel>>,
) {
    let intent_bar_background: Color = Color::rgba_u8(54, 2, 2, 255);
    let lure_bar_foreground: Color = Color::rgba_u8(42, 209, 56, 255);
    let repulse_bar_foreground: Color = Color::rgba_u8(54, 140, 56, 255);

    // spawn a status bar for Lure
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(35.0), Val::Percent(5.0)),
                position_type: PositionType::Absolute,
                position: UiRect {
                    left: Val::Px(25.0),
                    bottom: Val::Px(35.),
                    ..default()
                },
                ..default()
            },
            background_color: intent_bar_background.into(),
            ..default()
        })
        .insert(StatusBarWidget::new(0.0, 0., 1.))
        // spawn the moving inner bar
        .with_children(|outer| {
            outer
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(50.0), Val::Percent(100.0)),
                        position_type: PositionType::Absolute,
                        ..default()
                    },
                    background_color: lure_bar_foreground.into(),
                    ..default()
                })
                .insert(StatusBarInner);
        });
}

/// Update the [`StatusBarWidget`] with the current player health
fn set_status_bar(mut q: Query<&mut StatusBarWidget>, intent: Query<&Intent, With<Player>>) {
    for mut widget in q.iter_mut() {
        let intent = intent.single();
        let current_intent = intent.hp / intent.max;
        widget.set_progress(current_intent);
    }
}
