//! Displays available intent and selected option

use super::{
    bevy_widget_preview::{StatusBarDirection, StatusBarInner, StatusBarWidget},
    LeftPanel, UiStage,
};
use crate::player_interaction::intent::IntentPool;
use bevy::prelude::*;
use leafwing_abilities::pool::Pool;

pub fn setup_intent_status_bar(mut commands: Commands, ui_panel: Query<Entity, With<LeftPanel>>) {
    let intent_bar_background: Color = Color::rgba_u8(54, 2, 2, 255);
    let lure_bar_foreground: Color = Color::rgba_u8(42, 209, 56, 255);
    let repulse_bar_foreground: Color = Color::rgba_u8(54, 140, 56, 255);

    let left_panel = ui_panel.single();

    // spawn a status bar for Lure
    let lure_status_bar = commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(15.0), Val::Percent(22.0)),
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
        .insert(StatusBarWidget::new(
            0.0,
            0.,
            1.,
            crate::graphics::ui::intent_bar::StatusBarDirection::Vertical,
        ))
        // spawn the moving inner bar
        .with_children(|outer| {
            outer
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                        position_type: PositionType::Absolute,
                        ..default()
                    },
                    background_color: lure_bar_foreground.into(),
                    ..default()
                })
                .insert(StatusBarInner);
        })
        .id();

    commands.entity(left_panel).add_child(lure_status_bar);
}

/// Update the [`StatusBarWidget`] with the current player health
pub fn update_intent_status_bar(mut q: Query<&mut StatusBarWidget>, intent: Res<IntentPool>) {
    for mut widget in q.iter_mut() {
        if intent.is_changed() {
            let current_intent = intent.current().0;
            widget.set_status(current_intent);
        }
    }
}

/// Functionality for updating the intent status bar.
#[derive(Debug)]
pub struct IntentBarPlugin;

impl Plugin for IntentBarPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(UiStage::LayoutPopulation, setup_intent_status_bar)
            .add_system(update_intent_status_bar);
    }
}
