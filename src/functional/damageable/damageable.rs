/* Imports */
use bevy::prelude::*;
use crate::{components::planet::{Planet, PlayerPlanet}, systems::game::PlanetResource, utils::{audio::{game_sounds, play_audio}, logger}};

/// Some component that can be damaged
#[derive(Component)]
pub struct Damageable {
    pub health: f32,
    pub max_health: f32,
    pub flash_timer: Timer,
    pub callback: fn(&mut World) -> (),

    drop: Option<(PlanetResource, usize)>
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
    /// drop: (resource, amount)
    pub fn new(max_health: f32, drop: Option<(PlanetResource, usize)>, callback: fn(&mut World) -> ()) -> Self {
        Self {
            health: max_health, max_health, drop,
            flash_timer: Timer::from_seconds(0.1, TimerMode::Once),
            callback
        }
    }

    pub fn on_clicked(
        click: Trigger<Pointer<Down>>,
        mut commands: Commands,
        mut damage_events: ResMut<Events<DamageEvent>>,
        asset_server: Res<AssetServer>,
    ) {
        damage_events.send(DamageEvent { entity: click.entity(), damage: 10.0 });
        commands.entity(click.entity()).insert(Flashing);

        play_audio(&mut commands, &asset_server, game_sounds::tree::DAMAGE, false);
    }
    
    // Apply damage from events
    fn apply_damage(world: &mut World) {
        // Read damage events
        let damage_events = world.resource::<Events<DamageEvent>>();
        let mut reader = damage_events.get_cursor();
    
        // Collect entities to process
        let mut entities_to_process = Vec::new();
        for event in reader.read(&damage_events) {
            entities_to_process.push((event.entity, event.damage));
        }
    
        // Process entities
        for (entity, damage) in entities_to_process {
            let mut callback = None;
            let Ok(mut entity_mut) = world.get_entity_mut(entity) else { continue; };
            let Some(mut damageable) = entity_mut.get_mut::<Damageable>() else { continue; };

            damageable.health -= damage;
            if damageable.health <= 0.0 {
                let drop = damageable.drop.clone();
                callback = Some(damageable.callback);
                entity_mut.despawn();
            
                let Some((resource, amount)) = drop else { continue; };
                let Ok(mut planet) = world.query_filtered::<&mut Planet, With<PlayerPlanet>>().get_single_mut(world) else { continue; };
                planet.resources.add(resource, amount);
                logger::log::bright_green("resource", format!("Dropped {:?} x{}", resource, amount));
            }

            if let Some(callback) = callback {
                (callback)(world);
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
