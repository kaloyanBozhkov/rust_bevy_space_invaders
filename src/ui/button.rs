use bevy::prelude::*;

use crate::constants::Theme;

#[derive(Debug, Component, Clone)]
pub struct UIButton {
    pub id: String,
    pub text: String,
    pub click_count: i32,
    pub clickable_once: bool,
}

impl Default for UIButton {
    fn default() -> Self {
        UIButton {
            click_count: 0,
            id: "default id".to_string(),
            text: "default text".to_string(),
            clickable_once: true,
        }
    }
}

impl UIButton {
    pub fn create_many(
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        btns: &[UIButton],
        size: Option<Size>,
    ) {
        commands
            .spawn(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            })
            .with_children(|parent| {
                let size = match size {
                    Some(size) => size,
                    _ => Size::UNDEFINED,
                };

                for btn in btns {
                    parent
                        .spawn(ButtonBundle {
                            style: Style {
                                size,
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                border: UiRect::all(Val::Px(10.)),
                                margin: UiRect::bottom(Val::Px(20.)),
                                ..default()
                            },
                            background_color: Theme::PRIMARY_COLOR.into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                btn.text.to_string(),
                                TextStyle {
                                    font: asset_server.load("fonts/pixels_font.ttf"),
                                    font_size: 50.0,
                                    color: Color::rgb(255., 255., 255.),
                                },
                            ));
                        })
                        .insert(Interaction::default())
                        .insert(UIButton::from(btn.clone()));
                }
            });
    }
}
