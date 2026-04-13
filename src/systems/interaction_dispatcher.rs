use bevy::prelude::{Button, Changed, ImageNode, Query, With};
use bevy::prelude::Interaction;
use bevy::ui::RelativeCursorPosition;
use crate::systems::structures::level_structure::GameButton;

pub fn interact(
    mut query: Query<
        (&Interaction, &RelativeCursorPosition, &mut ImageNode, &GameButton),
        (Changed<Interaction>, With<Button>)
    >,
) {
    for(interaction, rel_pos, mut image, game_button) in &mut query {
        if rel_pos.cursor_over() {
            match *interaction {
                Interaction::Hovered => { image.image = game_button.im_hov_path.clone(); },
                Interaction::Pressed => { println!("Pressed") },
                Interaction::None => { image.image = game_button.im_path.clone(); }
            }
        } else {
            image.image = game_button.im_path.clone();
        }
    }
}