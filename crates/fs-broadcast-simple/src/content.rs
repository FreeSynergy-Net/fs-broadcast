//! What the window says, as blocks of the shared text view: the state, what works today, what comes.
//!
//! Pattern: Builder: a pure function from the words and the link state to blocks, so a test can read the window.

use fs_components_mf2::Mf2Texts;
use fs_components_widgets::{Block, Inline, ListItem};
use fs_components_words::{Args, Words};

use crate::link::Link;

/// How many entries the lists of `today-*` and `coming-*` have; a test checks that every id exists in both languages.
pub const TODAY: usize = 3;
/// See [`TODAY`].
pub const COMING: usize = 9;
/// The items that are not decided yet (more features): they say "planned, not scheduled" in their own words instead of "coming soon".
const OPEN: [usize; 1] = [9];

fn say(texts: &Mf2Texts, id: &str, args: &Args) -> String {
    texts.say(&id.into(), args).text().to_owned()
}

fn plain(texts: &Mf2Texts, id: &str) -> String {
    say(texts, id, &Args::new())
}

fn list(items: Vec<String>) -> Block {
    Block::List {
        ordered: false,
        start: 1,
        items: items
            .into_iter()
            .map(|t| ListItem::new(vec![Block::Paragraph(Inline::plain(t))]))
            .collect(),
    }
}

/// The start page below the header: the server state and what works today.
#[must_use]
pub fn start(texts: &Mf2Texts, link: &Link) -> Vec<Block> {
    let (state, hint) = match link {
        Link::Connected { contract } => (
            say(
                texts,
                "state-connected",
                &Args::new().text("contract", contract.as_str()),
            ),
            None,
        ),
        Link::Mismatch { found, expected } => (
            say(
                texts,
                "state-contract",
                &Args::new()
                    .text("found", found.as_str())
                    .text("expected", expected.as_str()),
            ),
            Some(plain(texts, "state-hint-contract")),
        ),
        Link::Down { code, detail } => {
            // One id per code of the client error; an unknown code reads as "unreachable".
            let id = match *code {
                "timeout" | "protocol" => format!("state-down-{code}"),
                _ => "state-down-unreachable".to_owned(),
            };
            (
                say(texts, &id, &Args::new().text("detail", detail.as_str())),
                Some(plain(texts, "state-hint-down")),
            )
        }
    };
    vec![
        Block::Quote(
            std::iter::once(Block::Paragraph(Inline::plain(state)))
                .chain(hint.map(|h| Block::Paragraph(Inline::plain(h))))
                .collect(),
        ),
        Block::Heading {
            level: 2,
            text: Inline::plain(plain(texts, "today-title")),
        },
        list(
            (1..=TODAY)
                .map(|n| plain(texts, &format!("today-{n}")))
                .collect(),
        ),
    ]
}

/// The page "what comes": every item locked with "coming soon".
#[must_use]
pub fn coming(texts: &Mf2Texts) -> Vec<Block> {
    let soon = plain(texts, "soon");
    vec![
        Block::Heading {
            level: 2,
            text: Inline::plain(plain(texts, "coming-title")),
        },
        list(
            (1..=COMING)
                .map(|n| {
                    let said = plain(texts, &format!("coming-{n}"));
                    if OPEN.contains(&n) {
                        said
                    } else {
                        format!("{said} ({soon})")
                    }
                })
                .collect(),
        ),
    ]
}

/// Both pages as one list, for the tests that read the whole window.
#[must_use]
pub fn blocks(texts: &Mf2Texts, link: &Link) -> Vec<Block> {
    let mut all = start(texts, link);
    all.extend(coming(texts));
    all
}

#[cfg(test)]
mod tests {
    use super::{COMING, OPEN, TODAY, blocks, plain};
    use crate::link::Link;
    use crate::words::words;
    use fs_components_words::Locale;

    fn flat(blocks: &[super::Block]) -> String {
        format!("{blocks:?}")
    }

    #[test]
    fn every_listed_item_exists_and_every_coming_item_is_locked() {
        let texts = words(&Locale::english());
        for n in 1..=TODAY {
            assert_ne!(plain(&texts, &format!("today-{n}")), "");
        }
        let said = flat(&blocks(
            &texts,
            &Link::Connected {
                contract: "c".into(),
            },
        ));
        assert_eq!(said.matches("coming soon").count(), COMING - OPEN.len());
        assert_eq!(said.matches("not scheduled").count(), OPEN.len());
    }

    #[test]
    fn a_missing_server_is_said_in_the_window_with_the_reason() {
        let texts = words(&Locale::english());
        let said = flat(&blocks(
            &texts,
            &Link::Down {
                code: "unreachable",
                detail: "nobody home".into(),
            },
        ));
        assert!(said.contains("No server is running."), "{said}");
        assert!(!said.contains("nobody home"), "{said}");
    }

    #[test]
    fn a_raw_sentence_of_a_library_never_reaches_the_window_in_either_language() {
        use fs_broadcast_client::Error;
        const RAW: &str =
            "The server could not be replaced (running: no server; needed: 0.0.2 protocol 1)";
        for locale in [Locale::english(), Locale::parse("de").unwrap()] {
            let texts = words(&locale);
            for error in [Error::Unreachable(RAW.into()), Error::Protocol(RAW.into())] {
                let said = flat(&blocks(&texts, &Link::of(Err(error))));
                assert!(!said.contains("could not be replaced"), "{said}");
                assert!(!said.contains("protocol 1"), "{said}");
            }
            let said = flat(&blocks(&texts, &Link::no_socket()));
            assert!(!said.contains("no socket"), "{said}");
        }
    }

    #[test]
    fn a_wrong_contract_is_said_in_the_window_with_the_reason() {
        let texts = words(&Locale::english());
        let said = flat(&blocks(
            &texts,
            &Link::Mismatch {
                found: "9.0".into(),
                expected: "0.1".into(),
            },
        ));
        assert!(
            said.contains("does not fit the server: it speaks 9.0, this window needs 0.1"),
            "{said}"
        );
    }

    #[test]
    fn the_open_items_are_named_without_a_promise() {
        let texts = words(&Locale::english());
        let said = flat(&blocks(
            &texts,
            &Link::Connected {
                contract: "c".into(),
            },
        ));
        assert!(said.contains("scenes and sources are wanted") && said.contains("no promise"));
    }
}
