use std::path::{Path, PathBuf};

use notify::{
    event::{DataChange, EventKind, ModifyKind},
    Config, Event, RecommendedWatcher, RecursiveMode, Watcher,
};

pub fn watch<P, C>(path: P, mut callback: C) -> notify::Result<()>
where
    P: AsRef<Path>,
    C: FnMut(Vec<PathBuf>),
{
    let (tx, rx) = std::sync::mpsc::channel();

    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;

    watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

    for res in rx {
        match res {
            Ok(Event {
                kind: EventKind::Modify(ModifyKind::Data(DataChange::Any)),
                paths,
                ..
            }) => callback(paths),
            Ok(event) => tracing::trace!("Ignored file watch event: {event:?}"),
            Err(e) => tracing::warn!("Watch error: {e:?}"),
        }
    }

    Ok(())
}
