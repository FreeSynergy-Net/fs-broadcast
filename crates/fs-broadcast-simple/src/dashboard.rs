//! The main view: a dashboard in the manner of streaming and restreaming tools (key figures, a preview surface, the
//! targets with their state, the schedule). Today no data flows, so it shows typed sample data, marked "preview".
//!
//! Pattern: Builder + Null object: [`Snapshot`] is the one typed model of what the view shows; [`Snapshot::sample`] is the
//! stand-in with neutral names (never real people or real addresses), and it is used only while no real snapshot exists.

use fs_components::blinc::layout::div::{Div, div};
use fs_components_mf2::Mf2Texts;
use fs_components_widgets::states::{Alert, EmptyState, Severity};
use fs_components_widgets::{Align, Block, Glyph, Inline, TextView};
use fs_components_words::{Args, Words};

/// The state of the input, as the contract's `StreamState` names it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Input {
    /// Nobody sends.
    Idle,
    /// Live.
    Live,
    /// The sender dropped for a short time; the targets stay connected.
    Interrupted,
    /// Anything else (checking, ended, error).
    Other,
}

/// The state of one target, as the contract's `TargetState` names it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Target {
    /// Switched off.
    Off,
    /// The stream is not live.
    Waiting,
    /// Connecting.
    Connecting,
    /// Sending.
    Sending,
    /// Lost, tries again.
    Retrying,
    /// The far end said no.
    Refused,
}

impl Target {
    fn id(self) -> &'static str {
        match self {
            Self::Off => "dash-target-off",
            Self::Waiting => "dash-target-waiting",
            Self::Connecting => "dash-target-connecting",
            Self::Sending => "dash-target-sending",
            Self::Retrying => "dash-target-retrying",
            Self::Refused => "dash-target-refused",
        }
    }
}

/// One target row.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TargetRow {
    /// The name the person gave it.
    pub name: String,
    /// Its state.
    pub state: Target,
    /// The bitrate it receives, in kbit/s.
    pub kbps: Option<u32>,
}

/// One scheduled broadcast.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Slot {
    /// The start, in UTC, as text.
    pub when: String,
    /// What it is called.
    pub title: String,
    /// How many targets it goes to.
    pub targets: u32,
}

/// Everything the dashboard shows.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Snapshot {
    /// The input.
    pub input: Input,
    /// Viewers, if known.
    pub viewers: Option<u32>,
    /// Incoming bitrate in kbit/s.
    pub kbps: Option<u32>,
    /// Seconds since going live.
    pub seconds: Option<u64>,
    /// The targets.
    pub targets: Vec<TargetRow>,
    /// The schedule.
    pub schedule: Vec<Slot>,
    /// `true` for [`Snapshot::sample`]: the view says "preview" and marks the figures.
    pub sample: bool,
}

fn say(texts: &Mf2Texts, id: &str, args: &Args) -> String {
    texts.say(&id.into(), args).text().to_owned()
}

fn plain(texts: &Mf2Texts, id: &str) -> String {
    say(texts, id, &Args::new())
}

impl Snapshot {
    /// The sample data (neutral names, one live input), only for the preview.
    #[must_use]
    pub fn sample(texts: &Mf2Texts) -> Self {
        let row = |n: u8, state, kbps| TargetRow {
            name: plain(texts, &format!("dash-sample-target-{n}")),
            state,
            kbps,
        };
        Self {
            input: Input::Live,
            viewers: Some(128),
            kbps: Some(4500),
            seconds: Some(4350),
            targets: vec![
                row(1, Target::Sending, Some(4500)),
                row(2, Target::Retrying, None),
                row(3, Target::Off, None),
            ],
            schedule: vec![
                Slot {
                    when: "2026-12-01 08:00".into(),
                    title: plain(texts, "dash-sample-slot-1"),
                    targets: 2,
                },
                Slot {
                    when: "2026-12-01 18:00".into(),
                    title: plain(texts, "dash-sample-slot-2"),
                    targets: 3,
                },
            ],
            sample: true,
        }
    }

    /// The snapshot to show: the real one when data flows, else the sample. Nothing of the sample survives a real one.
    #[must_use]
    pub fn shown(real: Option<Self>, texts: &Mf2Texts) -> Self {
        real.unwrap_or_else(|| Self::sample(texts))
    }
}

fn duration(seconds: u64) -> String {
    format!(
        "{:02}:{:02}:{:02}",
        seconds / 3600,
        seconds / 60 % 60,
        seconds % 60
    )
}

/// The tiles as (caption, value) pairs; a missing figure reads as a dash.
#[must_use]
pub fn tiles(snap: &Snapshot, texts: &Mf2Texts) -> Vec<(String, String)> {
    let input = match snap.input {
        Input::Live => "dash-input-live",
        Input::Idle => "dash-input-idle",
        Input::Interrupted => "dash-input-interrupted",
        Input::Other => "dash-input-other",
    };
    let dash = || "-".to_owned();
    vec![
        (plain(texts, "dash-tile-input"), plain(texts, input)),
        (
            plain(texts, "dash-tile-viewers"),
            snap.viewers.map_or_else(dash, |v| v.to_string()),
        ),
        (
            plain(texts, "dash-tile-bitrate"),
            snap.kbps.map_or_else(dash, |k| {
                say(
                    texts,
                    "dash-bitrate",
                    &Args::new().text("kbps", k.to_string()),
                )
            }),
        ),
        (
            plain(texts, "dash-tile-duration"),
            snap.seconds.map_or_else(dash, duration),
        ),
    ]
}

/// The targets as a table block (first row = header).
#[must_use]
pub fn targets_table(snap: &Snapshot, texts: &Mf2Texts) -> Block {
    let head = ["dash-col-target", "dash-col-state", "dash-col-bitrate"]
        .map(|id| Inline::plain(plain(texts, id)))
        .to_vec();
    let mut rows = vec![head];
    for t in &snap.targets {
        let kbps = t.kbps.map_or_else(
            || "-".to_owned(),
            |k| {
                say(
                    texts,
                    "dash-bitrate",
                    &Args::new().text("kbps", k.to_string()),
                )
            },
        );
        rows.push(vec![
            Inline::plain(t.name.as_str()),
            Inline::plain(plain(texts, t.state.id())),
            Inline::plain(kbps),
        ]);
    }
    Block::Table {
        align: vec![Align::Left, Align::Left, Align::Right],
        rows,
    }
}

/// The schedule as a table block, or one line when nothing is scheduled.
#[must_use]
pub fn schedule_block(snap: &Snapshot, texts: &Mf2Texts) -> Block {
    if snap.schedule.is_empty() {
        return Block::Paragraph(Inline::plain(plain(texts, "dash-schedule-empty")));
    }
    let head = ["dash-col-time", "dash-col-title", "dash-col-targets"]
        .map(|id| Inline::plain(plain(texts, id)))
        .to_vec();
    let mut rows = vec![head];
    for s in &snap.schedule {
        rows.push(vec![
            Inline::plain(s.when.as_str()),
            Inline::plain(s.title.as_str()),
            Inline::plain(s.targets.to_string()),
        ]);
    }
    Block::Table {
        align: vec![Align::Left, Align::Left, Align::Right],
        rows,
    }
}

fn heading(texts: &Mf2Texts, id: &str) -> Block {
    Block::Heading {
        level: 3,
        text: Inline::plain(plain(texts, id)),
    }
}

/// The view with every word already resolved, so building the elements needs no texts and can run on every layout.
#[derive(Clone, Debug)]
pub struct View {
    /// Title and text of the "preview" banner; `None` for real data.
    banner: Option<(String, String)>,
    figures: Block,
    tiles: Vec<Vec<Block>>,
    stage: (Block, String, String),
    targets: Vec<Block>,
    schedule: Vec<Block>,
}

impl View {
    /// Resolve the words for `snap`.
    #[must_use]
    pub fn new(snap: &Snapshot, texts: &Mf2Texts) -> Self {
        let mark = plain(texts, "dash-sample");
        let tiles = tiles(snap, texts)
            .into_iter()
            .map(|(caption, value)| {
                let caption = if snap.sample {
                    format!("{caption} ({mark})")
                } else {
                    caption
                };
                vec![
                    Block::Paragraph(Inline::plain(caption)),
                    Block::Heading {
                        level: 2,
                        text: Inline::plain(value),
                    },
                ]
            })
            .collect();
        Self {
            banner: snap.sample.then(|| {
                (
                    plain(texts, "dash-banner-title"),
                    plain(texts, "dash-banner-text"),
                )
            }),
            figures: heading(texts, "dash-figures"),
            tiles,
            stage: (
                heading(texts, "dash-stage-title"),
                plain(texts, "dash-stage-empty"),
                plain(texts, "dash-stage-text"),
            ),
            targets: vec![
                heading(texts, "dash-targets-title"),
                targets_table(snap, texts),
            ],
            schedule: vec![
                heading(texts, "dash-schedule-title"),
                schedule_block(snap, texts),
            ],
        }
    }

    /// Whether the banner "preview" is shown.
    #[must_use]
    pub fn is_preview(&self) -> bool {
        self.banner.is_some()
    }

    /// The elements.
    #[must_use]
    pub fn build(&self) -> Div {
        let mut page = div().class("fs-bc-page");
        if let Some((title, text)) = &self.banner {
            page = page.child(
                Alert::new(Severity::Info, text)
                    .title(title)
                    .id("bc-banner")
                    .build(),
            );
        }
        let mut row = div().class("fs-bc-row");
        for blocks in &self.tiles {
            row = row.child(
                div()
                    .class("fs-bc-tile")
                    .child(TextView::new(blocks.clone()).build()),
            );
        }
        // No live picture component exists yet: an empty-state surface stands in (a gap, listed in DESIGN.md).
        let (stage_title, empty, text) = &self.stage;
        let stage = div()
            .class("fs-bc-section")
            .child(TextView::new(vec![stage_title.clone()]).build())
            .child(
                EmptyState::new(empty)
                    .glyph(Glyph::FileVideo)
                    .text(text)
                    .id("bc-stage")
                    .build(),
            );
        let targets = div()
            .class("fs-bc-section")
            .child(TextView::new(self.targets.clone()).build());
        let schedule = div()
            .class("fs-bc-section")
            .child(TextView::new(self.schedule.clone()).build());
        page.child(TextView::new(vec![self.figures.clone()]).build())
            .child(row)
            .child(div().class("fs-bc-row").child(stage).child(targets))
            .child(schedule)
    }
}

/// The style sheet of the view: only theme variables, like the sheets of the components.
pub const STYLES: &str = r"
.fs-bc-page { display: flex; flex-direction: column; gap: var(--space-3); padding: var(--space-4); }
.fs-bc-row { display: flex; flex-direction: row; gap: var(--space-3); width: 100%; }
.fs-bc-tile { display: flex; flex-direction: column; flex-grow: 1; padding: var(--space-3); border-radius: var(--message-radius); border-width: 1px; border-color: var(--border); background: var(--surface); }
.fs-bc-section { display: flex; flex-direction: column; flex-grow: 1; gap: var(--space-2); padding: var(--space-3); border-radius: var(--message-radius); border-width: 1px; border-color: var(--border); background: var(--surface); }
";

#[cfg(test)]
mod tests {
    use super::{Input, Snapshot, Target, TargetRow, View, duration, targets_table, tiles};
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
        assert!(t[1..].iter().all(|(_, v)| v == "-"));
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

#[cfg(test)]
mod view_tests {
    use super::{Snapshot, View};
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
        assert!(!format!("{view:?}").contains("(sample)"));
    }
}
