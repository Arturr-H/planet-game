use bevy::{prelude::*, text::{FontSmoothing, TextSpanIter}};

pub struct SpawnInfoText(pub String);

#[derive(Resource)]
struct InfoText(Entity);

#[derive(Component)]
struct FadeOut { timer: Timer }

#[derive(Event)]
struct ShowInfoTextEvent(pub String);

pub struct InfoTextPlugin;
impl Plugin for InfoTextPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(InfoText(Entity::PLACEHOLDER))
            .add_event::<ShowInfoTextEvent>()
            .add_systems(Update, (Self::handle_fading_text_events, Self::update_fade_out))
            .add_systems(Startup, Self::setup);
    }
}
impl InfoTextPlugin {
    fn setup(
        mut commands: Commands,
        mut fading_text: ResMut<InfoText>,
        asset_server: Res<AssetServer>,
    ) -> () {
        commands.spawn((
            Node {
                width: Val::Vw(100.0),
                height: Val::Px(60.0),
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,

                position_type: PositionType::Absolute,
                bottom: Val::Px(0.0),

                ..Default::default()
            },
            Transform::from_xyz(0.0, 0.0, 10.0),
            PickingBehavior::IGNORE,
        )).with_children(|parent| {
            let text_entity = parent.spawn((
                Text::new(""),
                Visibility::Visible,
                TextColor(Color::WHITE),
                TextFont {
                    font: asset_server.load("fonts/ByteBounce.ttf"),
                    font_size: 24.0,
                    font_smoothing: FontSmoothing::None,
                },
                PickingBehavior::IGNORE,
            ));

            fading_text.0 = text_entity.id();
        });
    }

    fn handle_fading_text_events(
        mut commands: Commands,
        mut events: EventReader<ShowInfoTextEvent>,
        mut text_q: Query<(&mut Text, Entity)>,
        fading_text: ResMut<InfoText>,
    ) {
        for event in events.read() {
            // Update existing text and reset timer
            if let Ok((mut text, entity)) = text_q.get_mut(fading_text.0) {
                println!("Updating text: {}", event.0);
                text.0 = event.0.clone();

                commands.entity(entity)
                    .insert(FadeOut { timer: Timer::from_seconds(1.5, TimerMode::Once) });
            }
        }
    }

    fn update_fade_out(
        mut commands: Commands,
        time: Res<Time>,
        mut query: Query<(Entity, &mut FadeOut, &mut TextColor)>,
    ) {
        for (entity, mut fade_out, mut text_color) in &mut query {
            fade_out.timer.tick(time.delta());
            let elapsed = fade_out.timer.elapsed_secs();
            
            // Calculate alpha (1.0 for first second, then fade out over 1 second)
            let alpha = if elapsed < 0.5 {
                1.0
            } else {
                (1.5 - elapsed).clamp(0.0, 1.0)
            };

            text_color.set_alpha(alpha);


            if fade_out.timer.finished() {
                commands.entity(entity)
                    .remove::<FadeOut>();
            }
        }
    }
}
impl Command for SpawnInfoText {
    fn apply(self, world: &mut World) {
        world.send_event(ShowInfoTextEvent(self.0));
    }
}
