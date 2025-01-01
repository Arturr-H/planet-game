/* Imports */
use bevy::prelude::*;
use bevy::render::view::window;
use bevy::sprite::Sprite;

use crate::camera::OuterCamera;
use crate::utils::color::hex;
use crate::utils::sprite_bounds::point_in_sprite_bounds;
use crate::{RES_HEIGHT, RES_WIDTH};

/// Some component that can be damaged
#[derive(Component)]
pub struct Damageable {
    pub health: f32,
    pub max_health: f32,
    pub flash_timer: Timer,
}

/// Component for damage impact effect 
#[derive(Component)]
pub struct Flashing;

/// Event for damaging entities
#[derive(Event)]
pub struct DamageEvent {
    pub entity: Entity,
    pub damage: f32,
}


impl Damageable {
    pub fn new(max_health: f32) -> Self {
        Self {
            health: max_health, max_health,
            flash_timer: Timer::from_seconds(0.1, TimerMode::Once),
        }
    }

    pub fn on_clicked(
        click: Trigger<Pointer<Down>>,
        mut commands: Commands,
        mut damage_events: ResMut<Events<DamageEvent>>,
    ) {
        damage_events.send(DamageEvent { entity: click.entity(), damage: 10.0 });
        commands.entity(click.entity()).insert(Flashing);
    }
    
    // Apply damage from events
    fn apply_damage(
        mut commands: Commands,
        mut query: Query<(Entity, &mut Damageable)>,
        mut damage_events: EventReader<DamageEvent>,
    ) {
        for event in damage_events.read() {
            if let Ok((entity, mut damageable)) = query.get_mut(event.entity) {
                damageable.health -= event.damage;
                if damageable.health <= 0.0 {
                    commands.entity(entity).despawn();
                }
            }
        }
    }
    
    // Flash effect system
    fn handle_flashing(
        mut commands: Commands,
        mut query: Query<(Entity, &mut Damageable, &mut Sprite), With<Flashing>>,
        time: Res<Time>
    ) {
        for (entity, mut damageable, mut sprite) in query.iter_mut() {
            if damageable.flash_timer.tick(time.delta()).finished() {
                damageable.flash_timer.reset();
                commands.entity(entity).remove::<Flashing>();
                sprite.color = Color::WHITE;
            } else {
                sprite.color = Color::srgba(255., 255., 255., 1.);
            }
        }
    }
}

/// Plugin for enabling damageable entities
pub struct DamageablePlugin;
impl Plugin for DamageablePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<DamageEvent>()
            .add_systems(Update, (Damageable::apply_damage, Damageable::handle_flashing));
    }
}
