use bevy::prelude::*;

use crate::{GameState, sprites::SpriteSheet};

pub struct FadeoutPlugin;

#[derive(Component)]
struct ScreenFade {
    alpha: f32,
    sent: bool,
    next_state: GameState,
    timer: Timer
}

impl Plugin for FadeoutPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(fadeout);
    }
}

fn fadeout(
    // commands will be used to despawn the fade entity after the fade
    mut commands: Commands,
    // query for the fade entity (to despawn after the fade is done), 
    // ScreenFade component (for the timer and current current alpha), 
    // and the TextureAtlasSprite (to set the alpha of the fade entity)
    mut fade_query: Query<(Entity, &mut ScreenFade, &mut TextureAtlasSprite)>,
    // get the current game state so we can set the state once the fade is done
    mut state: ResMut<State<GameState>>,
    // we also need the time resource because a timer will be used
    time: Res<Time>
) {
    // for each entity, fade, and sprite in the fade_query
    for (entity, mut fade, mut sprite) in fade_query.iter_mut() {
        // tick the timer
        fade.timer.tick(time.delta());
        // if the timer is less than halfway done...
        if fade.timer.percent() < 0.5 {
            // increase the alpha (make it more transparent)...
            fade.alpha = fade.timer.percent() * 2.0;
        } else {
            // if it is more than halfway done decrease the alpha (make it less transparent)
            fade.alpha = fade.timer.percent_left() * 2.0;
        }
        // set the alpha
        sprite.color.set_a(fade.alpha);

        // if the timer is more than halfway done and the fade has not been sent ...
        if fade.timer.percent() > 0.5 && !fade.sent {
            // change the state
            state.set(fade.next_state).unwrap();
            // the fade has been sent
            fade.sent = true;
        }
        // if the timer has finished ...
        if fade.timer.just_finished() {
            // despawn the fade entity
            commands.entity(entity).despawn_recursive();
        }
    }
}

// possibly temporary function, cant use sprite_create beause it does not support 
pub fn create_fadeout(
    commands: &mut Commands, 
    next_state: GameState, 
    sprite_sheet: &Res<SpriteSheet>
) {
    let mut sprite = TextureAtlasSprite::new(0);
    sprite.color = Color::rgba(0.1, 0.1, 0.15, 0.0);
    sprite.custom_size = Some(Vec2::splat(1000000.0));

    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite,
            texture_atlas: sprite_sheet.0.clone(),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 999.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(ScreenFade{
            alpha: 0.0,
            sent: false,
            next_state,
            timer: Timer::from_seconds(1.0, false),
        })
        .insert(Name::new("Fadeout"));
}

