use bevy::prelude::*;

use crate::assets::UiAssets;

const FONT_SIZE: f32 = 18.;

#[derive(Component)]
pub struct ClickableButton;

pub fn get_button(
    commands: &mut Commands,
    height: Val,
    width: Val,
    margin: UiRect,
    image: &Handle<Image>,
) -> Entity {
    commands
        .spawn((
            ClickableButton,
            ButtonBundle {
                style: Style {
                    height,
                    width,
                    margin,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                image: UiImage::new(image.clone()),
                ..default()
            },
        ))
        .id()
}

pub fn button_click_animation(
    mut interactions: Query<
        (&Interaction, &mut Transform),
        (Changed<Interaction>, With<ClickableButton>),
    >,
) {
    for (interaction, mut transform) in interactions.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                transform.scale = Vec3::new(0.95, 0.95, 1.);
            }
            _ => {
                transform.scale = Vec3::splat(1.);
            }
        }
    }
}

pub fn get_text_bundle(text: &str, assets: &UiAssets) -> impl Bundle {
    TextBundle {
        text: Text::from_section(
            text,
            TextStyle {
                color: Color::WHITE,
                font: assets.font.clone(),
                font_size: FONT_SIZE,
                ..default()
            },
        ),
        ..default()
    }
}
