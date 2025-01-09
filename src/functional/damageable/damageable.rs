use std::f32::consts::PI;

/* Imports */
use bevy::{prelude::*, utils::HashMap};
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

/// This stores all damages done for entites,
/// because every time we do damage to an entity
/// we don't want to spawn a new label, but rather
/// update the existing one.
#[derive(Resource)]
pub struct DamageLabels {
    /// HashMap<Target entity (e.g tree), (Label entity, accumulated damage)>
    pub labels: HashMap<Entity, (Entity, f32)>,
}

/// Component for damage impact effect 
#[derive(Component)]
pub struct Flashing;

/// Event for damaging entities
#[derive(Event)]
pub struct DamageEvent {
    pub target_entity: Entity,
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
        mut damage_labels: ResMut<DamageLabels>,
        mut damage_text_q: Query<&mut AnimatedDamageText>,
        asset_server: Res<AssetServer>,
    ) {
        let target_entity = click.entity();
        let damage = (rand::random::<f32>() * 5.0 + 3.0).floor();
        damage_events.send(DamageEvent { target_entity, damage });

        /* Visual */
        commands.entity(click.entity()).insert(Flashing);
        AnimatedDamageText::spawn_damage_text(
            &mut commands,
            &asset_server,
            &mut damage_labels,
            &mut damage_text_q,
            target_entity,
            damage
        );
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
            entities_to_process.push((event.target_entity, event.damage));
        }
        damage_events.clear();
    
        // Process entities
        for (entity, damage) in entities_to_process {
            let mut callback = None;
            let Ok(mut entity_mut) = world.get_entity_mut(entity) else { continue; };
            let Some(mut damageable) = entity_mut.get_mut::<Damageable>() else { continue; };

            damageable.health -= damage;
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
    alpha_timer: Timer,
    timer: Timer,
    rotation: f32,
    speed: f32,
    damage: f32,
}

impl AnimatedDamageText {
    fn spawn_damage_text(
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        damage_labels: &mut ResMut<DamageLabels>,
        damage_text_q: &mut Query<&mut AnimatedDamageText>,
        target_entity: Entity,
        damage: f32
    ) {
        let mut rng = rand::thread_rng();
        let animated_text = AnimatedDamageText {
            timer: Timer::from_seconds(1.0, TimerMode::Once),
            alpha_timer: Timer::from_seconds(1.0, TimerMode::Once),
            rotation: PI / 2.0 + rng.gen_range(-0.4..0.4),
            speed: rng.gen_range(0.8..1.2),
            damage,
        };
    
        let (shell_entity, new_damage) = match damage_labels.labels.get_mut(&target_entity) {
            Some((prev_label_entity, accumulated_damage)) => {
                *accumulated_damage += damage;
                commands.get_entity(*prev_label_entity).map(|mut e| { e.despawn_descendants(); });
                (*prev_label_entity, *accumulated_damage)
            },
            None => {
                (Entity::PLACEHOLDER, damage)
            }
        };

        if let Ok(mut damage_text) = damage_text_q.get_mut(shell_entity) {
            damage_text.alpha_timer.reset();
        }

        let mut shell_entity = match commands.get_entity(shell_entity) {
            Some(e) => e,

            /* Create a new shell entity */
            None => {
                let mut shell_entity = None;
                commands.entity(target_entity).with_children(|parent| {
                    shell_entity = Some(parent.spawn((
                        animated_text,
                        InheritedVisibility::VISIBLE,
                        Transform::from_translation(Vec3::new(0.0, 32.0, 10.0)),
                    )).id());
                });

                match shell_entity.and_then(|e| commands.get_entity(e)) {
                    Some(e) => e,
                    None => return,
                }
            }
        };
        
        let chars = (new_damage as usize).to_string().chars().collect::<Vec<char>>();
        chars.iter().enumerate().for_each(|(i, c)| {
            const NUMBER_WIDTH: f32 = 16.0;
            let start = chars.len() as f32 * NUMBER_WIDTH / 2.0;
            let step = NUMBER_WIDTH * i as f32 * 0.75;

            shell_entity.with_children(|parent| {
                parent.spawn((
                    Sprite {
                        image: asset_server.load(format!("numbers/{}.png", c)),
                        ..Default::default()
                    },
                    Transform::from_translation(Vec3::new(- start + step, 0.0, 0.0))
                ));
            });
        });

        damage_labels.labels.insert(target_entity, (shell_entity.id(), new_damage));
    }

    fn update_text_animation(
        mut commands: Commands,
        time: Res<Time>,
        mut query: Query<(&Parent, Entity, &mut AnimatedDamageText, &mut Transform, &Children)>,
        mut query_opacity: Query<&mut Sprite>,
        mut damage_labels: ResMut<DamageLabels>,
    ) {
        for (target_entity, shell_entity, mut animated_text, mut transform, children) in query.iter_mut() {
            animated_text.timer.tick(time.delta());
            animated_text.alpha_timer.tick(time.delta());
    
            let progress = animated_text.timer.elapsed_secs() / animated_text.timer.duration().as_secs_f32();
            let alpha_timer_progress = animated_text.alpha_timer.elapsed_secs() / animated_text.alpha_timer.duration().as_secs_f32();
    
            let ext = 50.0 * (1.0 - progress) * time.delta_secs() * animated_text.speed;
            transform.translation.y += ext * animated_text.rotation.sin();
            transform.translation.x += ext * animated_text.rotation.cos();
            transform.translation.z -= ext / 1000.0;
            let alpha = ((2.0 - alpha_timer_progress * 2.0).min(1.0)).powi(2);
            for child in children.iter() {
                if let Ok(mut sprite) = query_opacity.get_mut(*child) {
                    sprite.color.set_alpha(alpha);
                }
            }
            if animated_text.alpha_timer.finished() {
                match commands.get_entity(shell_entity) {
                    Some(e) => {
                        e.despawn_recursive();
                        damage_labels.labels.remove(&target_entity.get());
                    },
                    None => (),
                }
            }
        }
    }
}

/// Plugin for enabling damageable entities
pub struct DamageablePlugin;
impl Plugin for DamageablePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(DamageLabels { labels: HashMap::default() })
            .add_event::<DamageEvent>()
            .add_systems(Update, (
                Damageable::apply_damage,
                Damageable::handle_flashing,
                AnimatedDamageText::update_text_animation,
            ));
    }
}
