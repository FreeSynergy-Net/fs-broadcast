# FreeSynergy Broadcast — show something, and bring it everywhere

**Record once, send it to many places, and keep your stream key secret.**

FreeSynergy Broadcast takes one live stream and sends it on to several targets at the same moment, without decoding it, and records it. You do not need a provider in the middle.

What is different here: the stream is passed on untouched, so quality is not lost and the machine does little work. A slow or dead target never slows the sender or the other targets. Keys are treated as secrets: the key you publish with is shown once and only a hash is kept, and keys never appear in logs or errors. If the sender drops, targets stay connected for a while and the stream continues when it returns.

## What it can do today

Latest tag: `v0.0.1` (the core only). The version in this tree is `0.0.2` (program with a window, a server and a frontend).

- **A window and a server (new in 0.0.2).** `fs-broadcast` makes sure the server runs and opens the frontend `simple`: name, icon, what works today, what comes, and a first look at the main view, a dashboard (key figures, stream preview, targets, schedule) and one navigation on the left (the sub pages say "coming soon"). The dashboard shows **sample data and says so**: the window shows sample data in this version, and the preview mark stands once at the top (banner and title); nothing in it is live. Everything that does not work yet says "coming soon". A missing, hanging or unfitting server is said in the window, never a grey window. Try `fs-broadcast --version`, `fs-broadcast --help`.
- Receive one RTMP stream and send it to many RTMP or RTMPS targets (the core, a library; not wired to the window).
- Record the stream to a folder as FLV.
- Check a target before going live, as far as that can be known.
- Try the core: `cargo run --example relay -- rtmp://host/app --key-env KEY` prints where to send and the one-time key, then forwards what it receives.

## What is not there yet

- The window shows no real stream and nothing in it can be used yet; the server answers only what it offers (nothing yet) and the contract version. The server keeps running after the window closes until it is stopped or replaced (no lease).
- Platform: Linux (Windows and macOS follow; the transport uses Unix sockets).
- Tested only on loopback with ffmpeg, never against a real streaming platform. Receiving is plain RTMP and stays on loopback by default.
- Handing a recording to Files is not built. A forwarding loop through another machine cannot be detected from here.
- Contract: draft (`streaming.capnp`, not locked). The basics are released with this draft contract. This frontend and server are its first consumers.
- Gaps in the main view, named: no stat tile, no stream-preview component and no list with status cells in fs-components yet (stand-ins, provisional), no command model for "Go live"; see DESIGN.md "Gaps".

## Part of FreeSynergy

Broadcast is the last step in the plan, in the media suite next to files. FreeSynergy is a family of programs that are free for everyone, forever, and that keep your data yours. The documentation, the manifesto and the overall plan live in [fs-documentation](https://github.com/FreeSynergy/fs-documentation); the reading version is at <https://freesynergy.github.io/fs-documentation/>. Start with the [manifesto](https://freesynergy.github.io/fs-documentation/manifesto/) if you want to know why we do this.


## For developers

- Workspace crates: `fs-broadcast-core` (streams, targets, keys as secrets, state; no media), `fs-broadcast-relay` (the forwarding, on `rml_rtmp` behind our own trait), `fs-broadcast-server` (the contract over a local socket), `fs-broadcast-client` (the contract as a client), `fs-broadcast-simple` (the frontend `simple`, a process of its own, depends only on the client and fs-components), `fs-broadcast-bin` (the command `fs-broadcast`: starter and `serve`).
- Engine and frontend are separate processes (ADR 0021, 0094 draft); the manifest of the frontend is `packaging/frontends/simple.metainfo.xml`.
- The contract is `streaming.capnp` in [fs-interfaces](https://github.com/FreeSynergy/fs-interfaces) (draft). Broadcast owns it; the capability key stays `streaming` (ADR 0087).
- Build and test: `cargo build --workspace` and `cargo test --workspace`. Set `FS_BROADCAST_NO_FFMPEG=1` to skip the tests that need ffmpeg.
- Design and decisions: [DESIGN.md](DESIGN.md).
- Licence: MIT OR Apache-2.0.

## Auf Deutsch

**Einmal aufnehmen, an viele Orte senden, den Stream-Schlüssel geheim halten.** FreeSynergy Broadcast nimmt einen Live-Strom entgegen, gibt ihn unverändert an mehrere Ziele weiter und zeichnet ihn auf.

**Was es heute kann.** Letzter Tag `v0.0.1` (nur der Kern). Der Stand in diesem Baum ist `0.0.2` (Programm mit Fenster, Server und Frontend). Neu ist ein Fenster mit Server: `fs-broadcast` startet den Server und das Frontend `simple` mit Name, Symbol, „Was es heute kann“, „Was kommt“ und einem ersten Blick auf die Hauptansicht, ein Dashboard (Kennzahlen, Strom-Vorschau, Ziele, Zeitplan). Das Dashboard zeigt **Beispieldaten und sagt es**: Das Fenster zeigt in dieser Fassung Beispieldaten; die Vorschau-Marke steht einmal oben (Banner + Titel); nichts darin ist live. Was noch nicht geht, sagt „kommt noch“. Fehlt der Server, hängt er oder passt er nicht, steht das im Fenster. Der Kern (Strom empfangen, an viele RTMP/RTMPS-Ziele weitergeben, aufzeichnen, Ziel prüfen) liegt als Bibliothek vor und ist noch nicht an das Fenster angeschlossen; heute ist darin nichts bedienbar.

**Plattform.** Linux (Windows und macOS folgen).

**Für Entwickler.** Frontend und Engine sind getrennte Prozesse; das Frontend hängt nur am Client und an fs-components. Bauen: `cargo build --workspace`, `cargo test --workspace`. Vertrag: Entwurf (`streaming.capnp`, nicht gesperrt). Die Grundfassung erscheint mit diesem Entwurf. Lücken der Hauptansicht (Ersatz-Komponenten, vorläufig): DESIGN.md „Gaps“. Lizenz: MIT OR Apache-2.0.
