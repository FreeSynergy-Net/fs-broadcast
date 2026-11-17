//! The window: name, the real icon (embedded PNG, own design), the content.
//!
//! Pattern: Adapter: our blocks in, a screen of the fs-components shell out; no widget is painted by hand.

use std::sync::{Arc, Mutex};

use fs_components::blinc::layout::div::div;
use fs_components::blinc::layout::image::image;
use fs_components::blinc::layout::widgets::scroll::{Scroll, ScrollPhysics};
use fs_components_app::{AppError, AppWidget, Element, Menu, MenuEntry, Screen, Status, Surface};
use fs_components_mf2::Mf2Texts;
use fs_components_widgets::{Block, Inline, TextView, styles};
use fs_components_words::{Args, Words};

use crate::content::{coming, start};
use crate::dashboard::{self, Snapshot, View};
use crate::link::Link;
use crate::nav::{ENTRIES, Sub};

/// The icon of the program, embedded in the binary.
const ICON: &[u8] = include_bytes!("../assets/fs-broadcast.png");

fn icon_uri() -> String {
    use base64::Engine;
    format!(
        "data:image/png;base64,{}",
        base64::engine::general_purpose::STANDARD.encode(ICON)
    )
}

/// What a screen shows.
enum Body {
    /// Text blocks, with the header (name, icon, goal) above them on the start page.
    Text { blocks: Vec<Block>, header: bool },
    /// The main view: the dashboard on its snapshot.
    Dashboard(Box<View>),
    /// A sub page of the navigation: "coming soon" and what will be there.
    Sub(Box<Sub>),
}

/// The one kind of screen of the window.
struct Home {
    name: String,
    tagline: String,
    body: Body,
}

/// Width of the box of the title, in pixels (the text view spreads over what it gets; a narrow box keeps the title at the icon).
const TITLE_WIDTH: f32 = 340.0;
/// How far the title box is pulled to the icon (Blinc counts margins in units of 4 px; the text view keeps room on its own left).
const TITLE_PULL: f32 = 13.0;
/// Side of the icon in the window, in pixels.
const ICON_SIZE: f32 = 64.0;

impl Home {
    fn header(&self) -> impl Element + use<> {
        // The icon of the program (own design, embedded; see packaging/ICON.md), as a data address Blinc loads; directly at
        // the title, vertically centred; `.no_fade()` so it is not pale.
        let icon = image(icon_uri()).size(ICON_SIZE, ICON_SIZE).no_fade();
        let title = div().w(TITLE_WIDTH).ml(-TITLE_PULL).child(
            TextView::new(vec![Block::Heading {
                level: 1,
                text: Inline::plain(self.name.as_str()),
            }])
            .build(),
        );
        div()
            .flex_row()
            .items_center()
            .gap(16.0)
            .child(icon)
            .child(title)
    }
}

impl Surface for Home {
    fn build(&self, width: f32, height: f32) -> Box<dyn Element> {
        let column: Box<dyn Element> = match &self.body {
            Body::Text { blocks, header } => {
                let page = div()
                    .flex_col()
                    .gap(4.0)
                    .p(24.0)
                    .w((width - 40.0).max(200.0));
                let body = TextView::new(blocks.clone()).build();
                if *header {
                    let goal =
                        TextView::new(vec![Block::Paragraph(Inline::plain(self.tagline.as_str()))])
                            .build();
                    Box::new(page.child(self.header()).child(goal).child(body))
                } else {
                    Box::new(page.child(body))
                }
            }
            Body::Dashboard(view) => Box::new(view.build()),
            Body::Sub(sub) => Box::new(sub.build()),
        };
        // The page scrolls when it is longer than the window, so nothing is cut off at the bottom.
        let physics = Arc::new(Mutex::new(ScrollPhysics::default()));
        Box::new(
            div().w(width).h(height).child(
                Scroll::with_physics(physics)
                    .w(width)
                    .h(height)
                    .content(column),
            ),
        )
    }

    fn styles(&self) -> String {
        format!("{}{}", styles(), dashboard::STYLES)
    }
}

/// The window for `link`, with the menus whose entries are all locked ("coming soon").
///
/// # Errors
/// The shell could not be made (a name that is empty, for example).
pub fn app(texts: &Mf2Texts, link: &Link) -> Result<AppWidget, AppError> {
    let say = |id: &str| texts.say(&id.into(), &Args::new()).text().to_owned();
    let soon = say("soon");
    let page = |body: Body| {
        Arc::new(Home {
            name: say("app-name"),
            tagline: say("app-tagline"),
            body,
        })
    };
    // No data flows through the contract yet, so the main view shows the sample, marked as a preview.
    let snapshot = Snapshot::shown(None, texts);
    // ONE navigation, the screen list of the shell on the left: the main view and its sub pages with a symbol each, then
    // "Start" and "What comes". Every entry leads to a screen.
    let mut screens: Vec<Screen> = ENTRIES
        .iter()
        .map(|entry| {
            let label = format!("{}  {}", entry.symbol, say(entry.label));
            let body = Sub::new(entry, texts).map_or_else(
                || Body::Dashboard(Box::new(View::new(&snapshot, texts))),
                |sub| Body::Sub(Box::new(sub)),
            );
            Screen::new(entry.id, label, page(body))
        })
        .collect();
    screens.push(Screen::new(
        "home",
        say("screen-start"),
        page(Body::Text {
            blocks: start(texts, link),
            header: true,
        }),
    ));
    screens.push(Screen::new(
        "coming",
        say("screen-coming"),
        page(Body::Text {
            blocks: coming(texts),
            header: false,
        }),
    ));
    let widget = AppWidget::new(say("app-name"), screens)?;
    let locked = |id: &str| {
        MenuEntry::new(id, format!("{} ({soon})", say(&format!("menu-{id}")))).disabled()
    };
    widget.set_menus(
        vec![Menu::new(
            say("menu-program"),
            vec![locked("golive"), locked("record"), locked("target")],
        )],
        |_| {},
    )?;
    widget.set_status(Some(Status::Info(say("status-ready"))));
    Ok(widget)
}
