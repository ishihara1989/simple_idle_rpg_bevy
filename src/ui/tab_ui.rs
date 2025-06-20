use bevy::prelude::*;
use crate::{UIState, TabButton, TabContent};

pub fn tab_button_system(
    mut interaction_query: Query<
        (&Interaction, &TabButton, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut tab_content_query: Query<(&mut Node, &TabContent)>,
    mut ui_state: ResMut<UIState>,
) {
    for (interaction, tab_button, mut background_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                ui_state.current_tab = tab_button.tab;
                
                for (mut node, tab_content) in &mut tab_content_query {
                    if tab_content.tab == ui_state.current_tab {
                        node.display = Display::Flex;
                    } else {
                        node.display = Display::None;
                    }
                }
                
                *background_color = BackgroundColor(Color::srgb(0.5, 0.5, 0.5));
            }
            Interaction::Hovered => {
                *background_color = BackgroundColor(Color::srgb(0.45, 0.45, 0.45));
            }
            Interaction::None => {
                *background_color = if tab_button.tab == ui_state.current_tab {
                    BackgroundColor(Color::srgb(0.4, 0.4, 0.4))
                } else {
                    BackgroundColor(Color::srgb(0.3, 0.3, 0.3))
                };
            }
        }
    }
}