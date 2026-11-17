//! The style sheet of the dashboard.
//!
//! Pattern: none (constants).

/// The style sheet of the view: only theme variables, like the sheets of the components.
pub const STYLES: &str = r"
.fs-bc-main { display: flex; flex-direction: column; flex-grow: 1; gap: var(--space-4); padding: var(--space-4); }
.fs-bc-row { display: flex; flex-direction: row; gap: var(--space-3); width: 100%; }
.fs-bc-tile { display: flex; flex-direction: column; flex-grow: 1; flex-basis: 0; min-width: 160px; gap: var(--space-2); padding: var(--space-3); border-radius: var(--message-radius); border-width: 1px; border-color: var(--border); background: var(--surface); }
.fs-bc-section { display: flex; flex-direction: column; flex-grow: 1; gap: var(--space-3); padding: var(--space-4); border-radius: var(--message-radius); border-width: 1px; border-color: var(--border); background: var(--surface); }
.fs-bc-tile-caption { color: var(--text-secondary); font-size: var(--text-sm); white-space: nowrap; }
.fs-bc-tile-value { color: var(--text-primary); font-size: 28px; font-weight: 700; white-space: nowrap; }
";
