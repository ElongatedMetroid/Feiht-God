use bevy::prelude::*;
use bevy_inspector_egui::{WorldInspectorPlugin, RegisterInspectable};

use crate::{player::{Player, EncounterTracker}, sprites::{Facing, AnimationTimer}};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        if cfg!(debug_assertions) {
            app.add_plugin(WorldInspectorPlugin::new())
                .register_type::<EncounterTracker>()
                .register_type::<AnimationTimer>()
                .register_inspectable::<Player>()
                .register_inspectable::<Facing>();
        }
    }
}