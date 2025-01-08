use std::f32::consts::PI;

/* Imports */
use bevy::prelude::*;
use rand::Rng;
use crate::{components::planet::{Planet, PlayerPlanet}, systems::game::PlanetResource, utils::{audio::{game_sounds, play_audio}, color::hex, logger}};

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
        mut query: Query<(Entity, &mut AnimatedDamageText, &mut Sprite, &mut Transform)>,
        asset_server: Res<AssetServer>,
    ) {
        let entity = click.entity();
        let damage = (rand::random::<f32>() * 5.0 + 3.0).floor();
        damage_events.send(DamageEvent { entity, damage });
        commands.entity(click.entity()).insert(Flashing);
        AnimatedDamageText::spawn_damage_text(&mut commands, &asset_server, entity, damage);

        play_audio(&mut commands, &asset_server, game_sounds::tree::DAMAGE, false);
    }
    
    // Apply damage from events
    fn apply_damage(world: &mut World) {
        // Read damage events
        let mut damage_events = world.resource_mut::<Events<DamageEvent>>();
        let mut reader = damage_events.get_cursor();
    
        // Collect entities to process
        let mut entities_to_process = Vec::new();
        for event in reader.read(&damage_events) {
            entities_to_process.push((event.entity, event.damage));
        }
        damage_events.clear();
    
        // Process entities
        for (entity, damage) in entities_to_process {
            let mut callback = None;
            let Ok(mut entity_mut) = world.get_entity_mut(entity) else { continue; };
            let Some(mut damageable) = entity_mut.get_mut::<Damageable>() else { continue; };

            damageable.health -= damage;
            println!("{} healt -{}", damageable.health, damage);
            if damageable.health <= 0.0 {
                let drop = damageable.drop.clone();
                callback = Some(damageable.callback);
                entity_mut.despawn_recursive();
            
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

#[derive(Component)]
pub struct AnimatedDamageText {
    timer: Timer,
    rotation: f32,
    speed: f32,
    damage: f32,
}

impl AnimatedDamageText {
    fn spawn_damage_text(
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        target_entity: Entity,
        damage: f32
    ) {
        let mut rng = rand::thread_rng();
    
        let texture = asset_server.load(format!("numbers/{damage}.png"));
        let animated_text = AnimatedDamageText {
            timer: Timer::from_seconds(1.0, TimerMode::Once),
            rotation: PI / 2.0 + rng.gen_range(-0.4..0.4),
            speed: rng.gen_range(0.8..1.2),
            damage,
        };
        commands.entity(target_entity).with_children(|parent| {
            parent.spawn((
                Sprite {
                    image: texture,
                    ..Default::default()
                },
                Transform::from_translation(Vec3::new(0.0, 32.0, 10.0)),
                animated_text,
            ));
        });
    }

    fn update_text_animation(
        mut commands: Commands,
        time: Res<Time>,
        mut query: Query<(Entity, &mut AnimatedDamageText, &mut Transform, &mut Sprite)>,
    ) {
        for (entity, mut animated_text, mut transform, mut sprite) in query.iter_mut() {
            animated_text.timer.tick(time.delta());
    
            let progress = animated_text.timer.elapsed_secs() / animated_text.timer.duration().as_secs_f32();
    
            let ext = 50.0 * (1.0 - progress) * time.delta_secs() * animated_text.speed;
            transform.translation.y += ext * animated_text.rotation.sin();
            transform.translation.x += ext * animated_text.rotation.cos();
            transform.translation.z -= ext / 1000.0;
            let alpha = ((2.0 - progress * 2.0).min(1.0)).powi(2);
            sprite.color.set_alpha(alpha);
            if animated_text.timer.finished() {
                // commands.entity(entity).despawn();
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
            .add_systems(Update, (
                Damageable::apply_damage,
                Damageable::handle_flashing,
                AnimatedDamageText::update_text_animation,
            ));
    }
}
