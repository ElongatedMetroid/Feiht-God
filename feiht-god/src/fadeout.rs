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
    mut commands: Commands,
    mut fade_query: Query<(Entity, &mut ScreenFade, &mut TextureAtlasSprite)>,
    mut state: ResMut<State<GameState>>,
    time: Res<Time>
) {
    for (entity, mut fade, mut sprite) in fade_query.iter_mut() {
        fade.timer.tick(time.delta());
        // if the timer is less than halfway done...
        if fade.timer.percent() < 0.5 {
            // increase the alpha ...
            fade.alpha = fade.timer.percent() * 2.0;
        } else {
            // if it is not less than halfway done decrease the alpha
            fade.alpha = fade.timer.percent_left() * 2.0;
        }
        sprite.color.set_a(fade.alpha);

        if fade.timer.percent() > 0.5 && !fade.sent {
            state.set(fade.next_state).unwrap();
            fade.sent = true;
        }
        if fade.timer.just_finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

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

