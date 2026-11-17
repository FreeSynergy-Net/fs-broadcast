//! The typed model of the dashboard: what it shows, and the sample for the preview.
//!
//! Pattern: Value objects plus a Null object (`Snapshot::sample`).

use fs_components_mf2::Mf2Texts;
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
    pub(crate) fn id(self) -> &'static str {
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
    /// `true` for [`Snapshot::sample`]: the view says "preview" in the banner and the title.
    pub sample: bool,
}

pub(crate) fn say(texts: &Mf2Texts, id: &str, args: &Args) -> String {
    texts.say(&id.into(), args).text().to_owned()
}

pub(crate) fn plain(texts: &Mf2Texts, id: &str) -> String {
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
