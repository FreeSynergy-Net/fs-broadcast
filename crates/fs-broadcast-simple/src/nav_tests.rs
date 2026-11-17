//! Tests of the navigation: every entry leads somewhere, there is one navigation, the title is the active entry, and a tile
//! never breaks inside a word.
//!
//! Pattern: none (tests). Tests: example-based, on both languages.

mod tests {
    use crate::dashboard::tiles;
    use crate::dashboard_model::{Snapshot, plain};
    use crate::nav::{ENTRIES, SCREEN_IDS, Sub};
    use crate::words::words;
    use fs_components_widgets::{Block, Inline};
    use fs_components_words::Locale;

    fn both() -> [Locale; 2] {
        [Locale::english(), Locale::parse("de").unwrap()]
    }

    #[test]
    fn every_entry_leads_to_a_screen_and_the_main_view_comes_first() {
        let ids: Vec<&str> = ENTRIES.iter().map(|e| e.id).collect();
        assert_eq!(ids[0], "main");
        assert_eq!(ids.len(), ENTRIES.len());
        let mut all = ids.clone();
        all.extend(["home", "coming"]);
        assert_eq!(all, SCREEN_IDS);
        let mut unique = all.clone();
        unique.sort_unstable();
        unique.dedup();
        assert_eq!(unique.len(), all.len());
    }

    #[test]
    fn every_sub_page_says_coming_soon_and_what_will_be_there_in_both_languages() {
        for locale in both() {
            let texts = words(&locale);
            for entry in &ENTRIES {
                let label = plain(&texts, entry.label);
                assert!(!label.is_empty() && label != entry.label, "{}", entry.id);
                match entry.what {
                    None => assert_eq!(entry.id, "main"),
                    Some(what) => {
                        let sub = Sub::new(entry, &texts).expect("a sub page");
                        // The title of the page is the label of its entry.
                        assert_eq!(
                            sub.title(),
                            &Block::Heading {
                                level: 1,
                                text: Inline::plain(label)
                            }
                        );
                        assert_ne!(plain(&texts, what), what, "{}", entry.id);
                        assert_ne!(plain(&texts, "sub-title"), "sub-title");
                    }
                }
            }
        }
    }

    #[test]
    fn there_is_no_second_navigation_inside_a_page() {
        for source in [
            include_str!("dashboard.rs"),
            include_str!("nav.rs"),
            include_str!("dashboard_styles.rs"),
        ] {
            assert!(!source.contains("fs-bc-nav"));
        }
        assert!(include_str!("dashboard_styles.rs").contains("white-space: nowrap"));
        {}
    }

    #[test]
    fn the_longest_tile_value_fits_the_fixed_tile_width_without_breaking_a_word() {
        // A tile is at least 160 px wide with 12 px padding on each side; a value of 28 px takes at most 16 px a character, and the text does not wrap.
        const FITS: usize = (160 - 24) / 16;
        for locale in both() {
            let texts = words(&locale);
            for (caption, value, unit) in tiles(&Snapshot::sample(&texts), &texts) {
                assert!(value.chars().count() <= FITS, "{caption}: {value}");
                assert!(!value.contains(' '), "{value}");
                assert!(!unit.contains(char::is_numeric), "{unit}");
            }
        }
    }
}
