# Changelog

Version = tag; all crates of the workspace carry the same version.

## 0.0.2

### English

- **Window and server (basics, workflow `grundfassung`).** New crates: `fs-broadcast-server` (the contract `streaming.capnp` draft 0.1 over a local socket through `fs-rpc` v0.0.2; `capabilities` answers a `CapabilityList` with no capability ids and the contract version `0.1`, and `supportReport`; everything else is `unimplemented`), `fs-broadcast-client` (connect with the version handshake, `Hello::fetch` with a limit of 15 s for the first conversation so a hanging instance ends with an error, contract check: same main level, minor level of the server not lower; errors have a code and a detail), `fs-broadcast-simple` (the frontend `simple` as a process of its own: name, icon, what works today, what comes, every element locked with "coming soon"; depends only on the client and fs-components v0.0.25), `fs-broadcast-bin` (the command `fs-broadcast`: starter, `serve`, `--version`, `--help`).
- **Main view (preview).** The page "Broadcast" is a dashboard in the manner of streaming tools: key figures, a stream-preview surface, the targets with state, the schedule. It shows typed sample data, marked "Preview" and "(sample)", until real data flows; missing components are stood in for and listed as gaps in DESIGN.md.
- **Starter.** `fs-broadcast` starts or replaces the server (handshake of `fs-rpc`), asks the frontend to check the contract (`--check`, exit code 3 = does not fit) and falls back to the standard frontend `simple` with a visible message. Errors in the window and on stderr are a code with parameters; the words are the program's own (MessageFormat 2, German and English).
- **Pins lifted so the tree holds one source per library:** `fs-objects` v0.0.1 to v0.0.6, `fs-interfaces` v0.0.1 to v0.0.27, new `fs-rpc` v0.0.2, `fs-types` v0.0.1 (through them). `.fs-lock-sources.toml` strict. Minimum Rust 1.89 for the workspace.
- **CI:** `check.yml` and `release.yml` call fs-release at `v0.0.10`. Linux only (`windows-targets` empty: Unix sockets).
- The real icon (`packaging/net.freesynergy.Broadcast.png`, `.desktop`, embedded in the window); `fs-broadcast` depends on the package `fs-broadcast-simple` (deb `depends`, rpm `requires`).
- Contract: draft (`streaming.capnp`, not locked). The basics are released with this draft contract.
- Not done: the window shows no real stream (the relay is not wired to the server), no lease (the server keeps running after the window closes until it is stopped or replaced).

### Deutsch

- **Fenster und Server (Grundfassung, Workflow `grundfassung`).** Neue Crates: `fs-broadcast-server` (der Vertrag `streaming.capnp` Entwurf 0.1 über einen lokalen Socket mit `fs-rpc` v0.0.2; `capabilities` antwortet mit einer `CapabilityList` ohne Fähigkeits-Kennungen und der Vertragsversion `0.1`, dazu `supportReport`; alles andere ist `unimplemented`), `fs-broadcast-client` (Verbinden mit Versions-Handschlag, `Hello::fetch` mit einer Frist von 15 s für das erste Gespräch, damit eine hängende Instanz mit einem Fehler endet, Vertragsprüfung: gleiche Hauptstufe, Nebenstufe des Servers nicht niedriger; Fehler haben Code und Detail), `fs-broadcast-simple` (das Frontend `simple` als eigener Prozess: Name, Symbol, was heute geht, was kommt, jedes Element mit „kommt noch“ gesperrt; hängt nur am Client und an fs-components v0.0.25), `fs-broadcast-bin` (der Befehl `fs-broadcast`: Starter, `serve`, `--version`, `--help`).
- **Hauptansicht (Vorschau).** Die Seite „Broadcast“ ist ein Dashboard nach Art der Streaming-Werkzeuge: Kennzahlen, eine Fläche für die Strom-Vorschau, die Ziele mit Zustand, der Zeitplan. Sie zeigt typisierte Beispieldaten, gekennzeichnet mit „Vorschau“ und „(Beispiel)“, bis echte Daten fließen; fehlende Komponenten sind durch Ersatz vertreten und als Lücken in DESIGN.md aufgelistet.
- **Starter.** `fs-broadcast` startet oder ersetzt den Server (Handschlag von `fs-rpc`), lässt das Frontend den Vertrag prüfen (`--check`, Rückgabecode 3 = passt nicht) und fällt mit sichtbarer Meldung auf das Standard-Frontend `simple` zurück. Fehler im Fenster und auf stderr sind ein Code mit Parametern; die Wörter gehören dem Programm (MessageFormat 2, Deutsch und Englisch).
- **Pins angehoben, damit der Baum je Bibliothek eine Quelle hat:** `fs-objects` v0.0.1 auf v0.0.6, `fs-interfaces` v0.0.1 auf v0.0.27, neu `fs-rpc` v0.0.2, `fs-types` v0.0.1 (durch diese). `.fs-lock-sources.toml` streng. Mindest-Rust 1.89 für den Arbeitsbereich.
- **CI:** `check.yml` und `release.yml` rufen fs-release bei `v0.0.10` auf. Nur Linux (`windows-targets` leer: Unix-Sockets).
- Das echte Symbol (`packaging/net.freesynergy.Broadcast.png`, `.desktop`, im Fenster eingebettet); `fs-broadcast` hängt vom Paket `fs-broadcast-simple` ab (deb `depends`, rpm `requires`).
- Vertrag: Entwurf (`streaming.capnp`, nicht gesperrt). Die Grundfassung erscheint mit diesem Entwurf.
- Nicht getan: das Fenster zeigt keinen echten Strom (das Relay ist nicht am Server angeschlossen), keine Lease (der Server läuft nach dem Schließen des Fensters weiter, bis er beendet oder ersetzt wird).

## 0.0.1

- The core: receive one RTMP stream, forward it untouched to many RTMP and RTMPS targets, record it as FLV, check a target before going live. Stage 1 of the design.
