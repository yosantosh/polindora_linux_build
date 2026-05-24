#[path = "../../src/theme.rs"]
mod theme;

use theme::{next_accent, next_theme, ACCENTS, THEMES};

#[test]
fn theme_cycle_advances_from_current_theme() {
    let next = next_theme("Polindora Dark");

    assert_eq!(next.name, "Midnight Blue");
    assert!(!next.description.is_empty());
}

#[test]
fn theme_cycle_wraps_after_last_theme() {
    let next = next_theme(THEMES.last().unwrap().name);

    assert_eq!(next.name, THEMES.first().unwrap().name);
}

#[test]
fn accent_cycle_advances_color_and_name() {
    let next = next_accent("Blue");

    assert_eq!(next.name, "Violet");
    assert_eq!(next.color, "#7867ff");
}

#[test]
fn accent_cycle_wraps_after_last_accent() {
    let next = next_accent(ACCENTS.last().unwrap().name);

    assert_eq!(next.name, ACCENTS.first().unwrap().name);
}
