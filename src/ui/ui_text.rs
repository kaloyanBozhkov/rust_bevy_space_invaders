use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct UIText {
    pub id: String,
}

impl UIText {
    pub fn create(
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        txt: String,
        font_size: f32,
        position: UiRect,
        text_id: String,
    ) {
        commands.spawn((
            // Create a TextBundle that has a Text with a single section.
            TextBundle::from_section(
                // Accepts a `String` or any type that converts into a `String`, such as `&str`
                txt,
                TextStyle {
                    font_size,
                    font: asset_server.load("fonts/pixels_font.ttf"),
                    color: Color::WHITE,
                },
            ) // Set the alignment of the Text
            .with_text_alignment(TextAlignment::CENTER)
            // Set the style of the TextBundle itself.
            .with_style(Style {
                position_type: PositionType::Absolute,
                position,
                ..default()
            }),
            UIText { id: text_id },
        ));
    }

    pub fn score(commands: &mut Commands, asset_server: &Res<AssetServer>, initial_score: i32) {
        UIText::create(
            commands,
            asset_server,
            "SCORE".to_string(),
            30.0,
            UiRect {
                right: Val::Px(10.0),
                top: Val::Px(5.0),
                ..default()
            },
            "score_header".to_string(),
        );

        UIText::create(
            commands,
            asset_server,
            initial_score.to_string(),
            40.0,
            UiRect {
                right: Val::Px(10.0),
                top: Val::Px(25.0),
                ..default()
            },
            "score_count".to_string(),
        );
    }
}
