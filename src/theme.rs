#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ThemePreset {
    pub name: &'static str,
    pub description: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AccentPreset {
    pub name: &'static str,
    pub color: &'static str,
}

pub const THEMES: [ThemePreset; 6] = [
    ThemePreset {
        name: "Polindora Dark",
        description: "Deep glass surfaces with electric blue glow",
    },
    ThemePreset {
        name: "Midnight Blue",
        description: "Cool navy layers with calm focus contrast",
    },
    ThemePreset {
        name: "Ocean Breeze",
        description: "Cyan highlights over dark aquatic glass",
    },
    ThemePreset {
        name: "Sunset Glow",
        description: "Warm amber and pink accents for evening focus",
    },
    ThemePreset {
        name: "Aurora",
        description: "Violet, cyan, and green atmospheric glow",
    },
    ThemePreset {
        name: "Light Mode",
        description: "Bright translucent panels for daytime work",
    },
];

pub const ACCENTS: [AccentPreset; 6] = [
    AccentPreset {
        name: "Blue",
        color: "#58a6ff",
    },
    AccentPreset {
        name: "Violet",
        color: "#7867ff",
    },
    AccentPreset {
        name: "Cyan",
        color: "#38d6ff",
    },
    AccentPreset {
        name: "Green",
        color: "#47e39b",
    },
    AccentPreset {
        name: "Amber",
        color: "#ffc857",
    },
    AccentPreset {
        name: "Pink",
        color: "#ff6bd6",
    },
];

pub fn next_theme(current: &str) -> ThemePreset {
    let index = THEMES
        .iter()
        .position(|theme| theme.name == current)
        .unwrap_or(0);

    THEMES[(index + 1) % THEMES.len()]
}

pub fn next_accent(current: &str) -> AccentPreset {
    let index = ACCENTS
        .iter()
        .position(|accent| accent.name == current)
        .unwrap_or(0);

    ACCENTS[(index + 1) % ACCENTS.len()]
}
