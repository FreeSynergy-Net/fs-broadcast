//! The style sheet of the dashboard.
//!
//! Pattern: none (constants).

/// The style sheet of the view: only theme variables, like the sheets of the components.
pub const STYLES: &str = r"
.fs-bc-page { display: flex; flex-direction: row; width: 100%; }
.fs-bc-nav { display: flex; flex-direction: column; gap: var(--space-1); width: 150px; flex-shrink: 0; padding: var(--space-4) var(--space-2); border-right-width: 1px; border-color: var(--border); }
.fs-bc-nav-item { display: flex; flex-direction: row; align-items: center; gap: var(--space-2); padding: var(--space-2); border-radius: var(--message-radius); }
.fs-bc-nav-active { background: var(--surface); border-width: 1px; border-color: var(--border); }
.fs-bc-nav-label { color: var(--text-primary); font-size: var(--text-sm); }
.fs-bc-main { display: flex; flex-direction: column; flex-grow: 1; gap: var(--space-4); padding: var(--space-4); }
.fs-bc-row { display: flex; flex-direction: row; gap: var(--space-3); width: 100%; }
.fs-bc-tile { display: flex; flex-direction: column; flex-grow: 1; flex-basis: 0; min-width: 110px; gap: var(--space-2); padding: var(--space-3); border-radius: var(--message-radius); border-width: 1px; border-color: var(--border); background: var(--surface); }
.fs-bc-section { display: flex; flex-direction: column; flex-grow: 1; gap: var(--space-3); padding: var(--space-4); border-radius: var(--message-radius); border-width: 1px; border-color: var(--border); background: var(--surface); }
";
