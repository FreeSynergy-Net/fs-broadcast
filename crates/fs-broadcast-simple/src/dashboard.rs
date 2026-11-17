//! The main view: a dashboard in the manner of streaming and restreaming tools (key figures, a preview surface, the
//! targets with their state, the schedule). Today no data flows, so it shows typed sample data, marked "preview".
//!
//! Pattern: Builder + Null object: [`Snapshot`] is the one typed model of what the view shows; [`Snapshot::sample`] is the
//! stand-in with neutral names (never real people or real addresses), and it is used only while no real snapshot exists.

use fs_components::blinc::layout::div::{Div, div};
use fs_components::blinc::layout::prelude::text;
use fs_components_mf2::Mf2Texts;
use fs_components_widgets::states::{Alert, EmptyState, Severity};
use fs_components_widgets::{Align, Block, Glyph, Inline, TextView};
use fs_components_words::Args;

pub use crate::dashboard_model::{Input, Slot, Snapshot, Target, TargetRow};
use crate::dashboard_model::{plain, say};

pub(crate) fn duration(seconds: u64) -> String {
    format!(
        "{:02}:{:02}:{:02}",
        seconds / 3600,
        seconds / 60 % 60,
        seconds % 60
    )
}

/// The tiles as (caption, value) pairs; a missing figure reads as a dash.
#[must_use]
pub fn tiles(snap: &Snapshot, texts: &Mf2Texts) -> Vec<(String, String, String)> {
    let input = match snap.input {
        Input::Live => "dash-input-live",
        Input::Idle => "dash-input-idle",
        Input::Interrupted => "dash-input-interrupted",
        Input::Other => "dash-input-other",
    };
    let dash = || "-".to_owned();
    // The number and its unit are two lines, so a tile never breaks inside a word.
    vec![
        (
            plain(texts, "dash-tile-input"),
            plain(texts, input),
            String::new(),
        ),
        (
            plain(texts, "dash-tile-viewers"),
            snap.viewers.map_or_else(dash, |v| v.to_string()),
            String::new(),
        ),
        (
            plain(texts, "dash-tile-bitrate"),
            snap.kbps.map_or_else(dash, |k| k.to_string()),
            snap.kbps
                .map_or_else(String::new, |_| plain(texts, "dash-unit-kbps")),
        ),
        (
            plain(texts, "dash-tile-duration"),
            snap.seconds.map_or_else(dash, duration),
            String::new(),
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
        level: 2,
        text: Inline::plain(plain(texts, id)),
    }
}

/// The view with every word already resolved, so building the elements needs no texts and can run on every layout.
#[derive(Clone, Debug)]
pub struct View {
    /// Title and text of the "preview" banner; `None` for real data.
    banner: Option<(String, String)>,
    title: Block,
    figures: Block,
    tiles: Vec<(String, String, String)>,
    stage: (Block, String, String),
    targets: Vec<Block>,
    schedule: Vec<Block>,
}

impl View {
    /// Resolve the words for `snap`.
    #[must_use]
    pub fn new(snap: &Snapshot, texts: &Mf2Texts) -> Self {
        // The preview mark stands once in the banner and once in the title, never at each figure.
        let tiles = tiles(snap, texts);
        let title_id = if snap.sample {
            "dash-title-preview"
        } else {
            "dash-title"
        };
        Self {
            banner: snap.sample.then(|| {
                (
                    plain(texts, "dash-banner-title"),
                    plain(texts, "dash-banner-text"),
                )
            }),
            title: Block::Heading {
                level: 1,
                text: Inline::plain(plain(texts, title_id)),
            },
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
        let mut page = div().class("fs-bc-main");
        page = page.child(TextView::new(vec![self.title.clone()]).build());
        if let Some((title, text)) = &self.banner {
            page = page.child(
                Alert::new(Severity::Info, text)
                    .title(title)
                    .id("bc-banner")
                    .build(),
            );
        }
        let mut row = div().class("fs-bc-row");
        // Tiles use plain text elements, not the text view: it breaks a short value inside a word when the box is narrow.
        for (caption, value, unit) in &self.tiles {
            let mut tile = div()
                .class("fs-bc-tile")
                .child(text(caption.clone()).class("fs-bc-tile-caption"))
                .child(text(value.clone()).class("fs-bc-tile-value"));
            if !unit.is_empty() {
                tile = tile.child(text(unit.clone()).class("fs-bc-tile-caption"));
            }
            row = row.child(tile);
        }
        // No live picture component exists yet: an empty-state surface stands in (a gap, listed in DESIGN.md).
        let (stage_title, empty, stage_text) = &self.stage;
        let stage = div()
            .class("fs-bc-section")
            .child(TextView::new(vec![stage_title.clone()]).build())
            .child(
                EmptyState::new(empty)
                    .glyph(Glyph::FileVideo)
                    .text(stage_text)
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
            .child(stage)
            .child(targets)
            .child(schedule)
    }
}

pub use crate::dashboard_styles::STYLES;
