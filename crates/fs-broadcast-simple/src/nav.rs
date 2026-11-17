//! The one navigation of the window: the entries of the shell's screen list on the left. The main view and its sub pages come
//! first, then "Start" and "What comes". Every entry leads to a screen: the main view, or an honest page that says
//! "coming soon" and what will be there (never a dead entry, never a second navigation inside a page).
//!
//! Pattern: Value objects (a table of entries) + Builder (a page from words).

use fs_components::blinc::layout::div::{Div, div};
use fs_components_mf2::Mf2Texts;
use fs_components_widgets::states::EmptyState;
use fs_components_widgets::{Block, Glyph, Inline, TextView};

use crate::dashboard_model::plain;

/// One entry of the navigation.
#[derive(Clone, Copy, Debug)]
pub struct Entry {
    /// The id of the screen (also `--screen`).
    pub id: &'static str,
    /// The symbol before the label (a character every font has; the screen list takes text only).
    pub symbol: &'static str,
    /// The id of the label in the words; it is also the title of the screen.
    pub label: &'static str,
    /// The glyph of the empty surface of a sub page.
    pub glyph: Glyph,
    /// The id of the sentence that says what will be on the page; `None` for the main view (it has content).
    pub what: Option<&'static str>,
}

/// The entries, main view first.
pub const ENTRIES: [Entry; 6] = [
    Entry {
        id: "main",
        symbol: "\u{25CF}",
        label: "nav-broadcast",
        glyph: Glyph::Home,
        what: None,
    },
    Entry {
        id: "targets",
        symbol: "\u{2261}",
        label: "nav-targets",
        glyph: Glyph::Transfers,
        what: Some("sub-targets-text"),
    },
    Entry {
        id: "recordings",
        symbol: "\u{25A4}",
        label: "nav-recordings",
        glyph: Glyph::FileVideo,
        what: Some("sub-recordings-text"),
    },
    Entry {
        id: "input",
        symbol: "\u{25BC}",
        label: "nav-input",
        glyph: Glyph::Download,
        what: Some("sub-input-text"),
    },
    Entry {
        id: "log",
        symbol: "\u{25A6}",
        label: "nav-log",
        glyph: Glyph::History,
        what: Some("sub-log-text"),
    },
    Entry {
        id: "settings",
        symbol: "\u{25C6}",
        label: "nav-settings",
        glyph: Glyph::Settings,
        what: Some("sub-settings-text"),
    },
];

/// The ids of all screens, in the order of the list (the entries, then start and what comes).
pub const SCREEN_IDS: [&str; 8] = [
    "main",
    "targets",
    "recordings",
    "input",
    "log",
    "settings",
    "home",
    "coming",
];

/// A sub page with its words resolved: title, "coming soon" and what will be here.
#[derive(Clone, Debug)]
pub struct Sub {
    title: Block,
    glyph: Glyph,
    soon: String,
    what: String,
}

impl Sub {
    /// The page of `entry`, or `None` for the main view.
    #[must_use]
    pub fn new(entry: &Entry, texts: &Mf2Texts) -> Option<Self> {
        let what = entry.what?;
        Some(Self {
            title: Block::Heading {
                level: 1,
                text: Inline::plain(plain(texts, entry.label)),
            },
            glyph: entry.glyph,
            soon: plain(texts, "sub-title"),
            what: plain(texts, what),
        })
    }

    /// The title of the page (the label of its entry).
    #[must_use]
    pub fn title(&self) -> &Block {
        &self.title
    }

    /// The elements: the title, then the empty surface with symbol, "coming soon" and the explanation.
    #[must_use]
    pub fn build(&self) -> Div {
        div()
            .class("fs-bc-main")
            .child(TextView::new(vec![self.title.clone()]).build())
            .child(
                div().class("fs-bc-section").child(
                    EmptyState::new(&self.soon)
                        .glyph(self.glyph)
                        .text(&self.what)
                        .id("bc-sub")
                        .build(),
                ),
            )
    }
}
