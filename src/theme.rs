/// Rec. 709 luma coefficients for luminance calculation.
pub const LUMA_R: f32 = 0.2126;
pub const LUMA_G: f32 = 0.7152;
pub const LUMA_B: f32 = 0.0722;

/// Normalized RGB color with components in [0.0, 1.0].
#[derive(Debug, Clone, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    pub const fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }

    /// Adjust saturation. < 1.0 desaturates (toward gray),
    /// > 1.0 boosts (more vivid). 1.0 returns unchanged.
    #[must_use]
    pub fn with_saturation(&self, saturation: f32) -> Self {
        let gray = self.r * LUMA_R + self.g * LUMA_G + self.b * LUMA_B;
        Self {
            r: (gray + (self.r - gray) * saturation).clamp(0.0, 1.0),
            g: (gray + (self.g - gray) * saturation).clamp(0.0, 1.0),
            b: (gray + (self.b - gray) * saturation).clamp(0.0, 1.0),
        }
    }
}

/// A named monochromatic theme.
#[derive(Debug, Clone)]
pub struct Theme {
    pub name: &'static str,
    pub description: &'static str,
    pub color: Color,
    /// Dominant wavelength range in nm.
    pub wavelength_range: (u16, u16),
}

static THEMES: &[Theme] = &[
    Theme {
        name: "military",
        description: "Night-vision tactical green",
        color: Color::new(0.2, 1.0, 0.4), // P39 phosphor #33FF66
        wavelength_range: (530, 560),
    },
    Theme {
        name: "green",
        description: "Classic P1 green CRT",
        color: Color::new(0.29, 1.0, 0.0), // P1 phosphor 525nm #4AFF00
        wavelength_range: (520, 530),
    },
    Theme {
        name: "amber",
        description: "Classic amber CRT",
        color: Color::new(1.0, 0.71, 0.0), // P3 phosphor 602nm #FFB400
        wavelength_range: (598, 608),
    },
    Theme {
        name: "alert",
        description: "Red warning lights",
        color: Color::new(1.0, 0.0, 0.0), // Pure red
        wavelength_range: (620, 680),
    },
    Theme {
        name: "cyber",
        description: "Neon futuristic cyan",
        color: Color::new(0.0, 1.0, 1.0), // Pure cyan #00FFFF
        wavelength_range: (485, 500),
    },
    Theme {
        name: "arctic",
        description: "Cold ice blue",
        color: Color::new(0.0, 0.7, 1.0), // Vivid azure
        wavelength_range: (460, 480),
    },
    Theme {
        name: "cobalt",
        description: "Deep industrial blue",
        color: Color::new(0.0, 0.42, 1.0), // Cobalt #0047AB normalized
        wavelength_range: (450, 470),
    },
    Theme {
        name: "void",
        description: "Deep cosmic purple",
        color: Color::new(0.5, 0.0, 1.0), // Deep violet
        wavelength_range: (400, 430),
    },
    Theme {
        name: "toxic",
        description: "Radioactive yellow-green",
        color: Color::new(0.44, 1.0, 0.19), // Biohazard #61DE2A normalized
        wavelength_range: (550, 570),
    },
    Theme {
        name: "infrared",
        description: "Thermal camera magenta",
        color: Color::new(1.0, 0.0, 1.0), // Pure magenta #FF00FF
        wavelength_range: (620, 700),
    },
    Theme {
        name: "rose",
        description: "Soft lo-fi pink",
        color: Color::new(1.0, 0.4, 0.8), // Lo-fi pink #FF66CC
        wavelength_range: (600, 650),
    },
    Theme {
        name: "sepia",
        description: "Old photograph warmth",
        color: Color::new(1.0, 0.59, 0.18), // #704214 normalized for tint
        wavelength_range: (580, 620),
    },
    Theme {
        name: "walnut",
        description: "Dark stained wood",
        color: Color::new(0.7, 0.35, 0.1), // Dark warm brown
        wavelength_range: (580, 610),
    },
    Theme {
        name: "white",
        description: "Classic P4 white CRT",
        color: Color::new(1.0, 1.0, 1.0),
        wavelength_range: (380, 700),
    },
];

/// Return all built-in themes.
#[must_use]
pub fn builtin_themes() -> &'static [Theme] {
    THEMES
}

/// Look up a theme by name (case-insensitive).
#[must_use]
pub fn find_theme(name: &str) -> Option<&'static Theme> {
    THEMES.iter().find(|t| t.name.eq_ignore_ascii_case(name))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builtin_themes_count() {
        assert_eq!(builtin_themes().len(), 14);
    }

    #[test]
    fn find_theme_exact() {
        let t = find_theme("military").unwrap();
        assert_eq!(t.name, "military");
    }

    #[test]
    fn find_all_themes_by_name() {
        let names = [
            "military", "green", "amber", "alert", "cyber", "arctic", "cobalt", "void", "toxic",
            "infrared", "rose", "sepia", "walnut", "white",
        ];
        for name in names {
            assert!(find_theme(name).is_some(), "Theme '{name}' not found");
        }
    }

    #[test]
    fn find_theme_case_insensitive() {
        assert_eq!(find_theme("AMBER").unwrap().name, "amber");
        assert_eq!(find_theme("Cyber").unwrap().name, "cyber");
        assert_eq!(find_theme("WHITE").unwrap().name, "white");
    }

    #[test]
    fn find_theme_unknown() {
        assert!(find_theme("nonexistent").is_none());
    }

    #[test]
    fn all_colors_normalized() {
        for theme in builtin_themes() {
            assert!(
                (0.0..=1.0).contains(&theme.color.r)
                    && (0.0..=1.0).contains(&theme.color.g)
                    && (0.0..=1.0).contains(&theme.color.b),
                "Theme '{}' has out-of-range color",
                theme.name
            );
        }
    }

    #[test]
    fn all_wavelengths_valid() {
        for theme in builtin_themes() {
            let (lo, hi) = theme.wavelength_range;
            assert!(
                lo < hi,
                "Theme '{}' has invalid wavelength range",
                theme.name
            );
            assert!(
                lo >= 380 && hi <= 700,
                "Theme '{}' wavelength outside visible spectrum",
                theme.name
            );
        }
    }

    #[test]
    fn all_names_unique() {
        let mut names: Vec<&str> = builtin_themes().iter().map(|t| t.name).collect();
        names.sort();
        names.dedup();
        assert_eq!(names.len(), builtin_themes().len());
    }

    #[test]
    fn saturation_desaturate() {
        let green = Color::new(0.0, 1.0, 0.0);
        let muted = green.with_saturation(0.0);
        let gray = LUMA_G; // pure green luminance = LUMA_G
        assert!((muted.r - gray).abs() < 0.001);
        assert!((muted.g - gray).abs() < 0.001);
        assert!((muted.b - gray).abs() < 0.001);
    }

    #[test]
    fn saturation_unchanged() {
        let color = Color::new(0.5, 0.3, 0.8);
        let same = color.with_saturation(1.0);
        assert!((same.r - color.r).abs() < 0.001);
        assert!((same.g - color.g).abs() < 0.001);
        assert!((same.b - color.b).abs() < 0.001);
    }

    #[test]
    fn saturation_boost() {
        let color = Color::new(0.2, 0.8, 0.4);
        let vivid = color.with_saturation(1.5);
        assert!(vivid.r < color.r);
        assert!(vivid.g > color.g);
    }

    #[test]
    fn saturation_clamps() {
        let color = Color::new(0.0, 1.0, 0.0);
        let boosted = color.with_saturation(2.0);
        assert!(boosted.r >= 0.0 && boosted.r <= 1.0);
        assert!(boosted.g >= 0.0 && boosted.g <= 1.0);
        assert!(boosted.b >= 0.0 && boosted.b <= 1.0);
    }

    #[test]
    fn color_equality() {
        assert_eq!(Color::new(1.0, 0.0, 0.0), Color::new(1.0, 0.0, 0.0));
        assert_ne!(Color::new(1.0, 0.0, 0.0), Color::new(0.0, 1.0, 0.0));
    }
}
