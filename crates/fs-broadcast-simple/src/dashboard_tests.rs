//! Tests of the dashboard model and view.
//!
//! Pattern: none (tests). Tests: example-based, on the sample and on a real snapshot.

mod tests {
    use crate::dashboard::{Input, Snapshot, Target, TargetRow, duration, targets_table, tiles};
    use crate::words::words;
    use fs_components_words::Locale;

    #[test]
    fn the_sample_is_marked_and_a_real_snapshot_replaces_it_completely() {
        let texts = words(&Locale::english());
        let sample = Snapshot::sample(&texts);
        assert!(sample.sample);
        let real = Snapshot {
            input: Input::Idle,
            viewers: None,
            kbps: None,
            seconds: None,
            targets: vec![TargetRow {
                name: "mine".into(),
                state: Target::Off,
                kbps: None,
            }],
            schedule: vec![],
            sample: false,
        };
        let shown = Snapshot::shown(Some(real.clone()), &texts);
        assert_eq!(shown, real);
        assert!(!format!("{shown:?}").contains("Sample"));
        assert!(Snapshot::shown(None, &texts).sample);
    }

    #[test]
    fn tiles_say_a_dash_for_what_is_unknown_and_time_reads_as_clock() {
        let texts = words(&Locale::english());
        let none = Snapshot {
            sample: false,
            viewers: None,
            kbps: None,
            seconds: None,
            ..Snapshot::sample(&texts)
        };
        let t = tiles(&none, &texts);
        assert_eq!(t.len(), 4);
        assert!(t[1..].iter().all(|(_, v, u)| v == "-" && u.is_empty()));
        assert_eq!(duration(4350), "01:12:30");
    }

    #[test]
    fn the_sample_has_neutral_names_in_both_languages_and_a_target_table_with_header() {
        for locale in [Locale::english(), Locale::parse("de").unwrap()] {
            let texts = words(&locale);
            let text = format!("{:?}", targets_table(&Snapshot::sample(&texts), &texts));
            assert!(
                text.contains("Beispielziel") || text.contains("Sample target"),
                "{text}"
            );
        }
    }
}

mod view_tests {
    use crate::dashboard::{Snapshot, View};
    use crate::words::words;
    use fs_components_words::Locale;

    #[test]
    fn the_preview_banner_shows_for_the_sample_and_never_for_real_data() {
        let texts = words(&Locale::english());
        let sample = View::new(&Snapshot::sample(&texts), &texts);
        assert!(sample.is_preview());
        let real = Snapshot {
            sample: false,
            ..Snapshot::sample(&texts)
        };
        let view = View::new(&real, &texts);
        assert!(!view.is_preview());
        assert!(!format!("{view:?}").contains("Preview"));
    }
}
