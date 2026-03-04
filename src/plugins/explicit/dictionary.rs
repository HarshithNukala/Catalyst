use std::result;
use std::sync::{Arc, RwLock};
use anyhow::Ok;
use async_trait::async_trait;
use futures::lock::Mutex;
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;
use arboard::Clipboard;
use rusqlite::{Connection};
use anyhow::Result;

use crate::core::plugin::{Plugin, PluginContext, Trigger};
use crate::core::model::{Action, BuiltInIcon, ResultIcon, ResultItem};

pub struct DictionaryPlugin {
    connection: Mutex<Connection>,
}

impl DictionaryPlugin {
    pub fn new() -> Self {
        Self {
            connection: Mutex::new(Connection::open("assets/dictionary.db").unwrap()),
        }
    }
}

#[async_trait]
impl Plugin for DictionaryPlugin {
    fn id(&self) -> &str {
        "dictionary"
    }
    fn name(&self) -> &str {
        "Dictionary Plugin"
    }
    fn description(&self) -> &str {
        "A plugin for getting dictionary definitions."
    }
    fn trigger(&self) -> Trigger {
        Trigger::Explicit { keyword: "def".to_string() }
    }
    async fn search(&self, query: &str, _context: &PluginContext) -> Vec<ResultItem> {
        let mut results = Vec::new();
        let conn = self.connection.lock().await;
        let mut stmt = conn.prepare(
    "SELECT word, definition FROM definitions WHERE word LIKE ?1 COLLATE NOCASE LIMIT 10"
).unwrap();
        if query.is_empty() {
            return results;
        }
        let pattern = format!("{}%", query);
        let rows = stmt.query_map([&pattern], |row| {
            rusqlite::Result::Ok((
                row.get::<_, String>(0)?,
                row.get::<_, Option<String>>(1)?
            ))
        }).unwrap();
        for row in rows {
            if let rusqlite::Result::Ok((word, definition)) = row {
                let def = definition.unwrap_or_default();
                let subtitle = def.clone();
                results.push(
                    ResultItem::new(
                        &format!("{}", word),
                        word.clone(),
                        Action::CopyToClipboard(def),
                        self.id().to_string()
                    )
                    .with_subtitle(subtitle)
                    .with_icon(ResultIcon::BuiltIn(BuiltInIcon::Dictionary))
                    .with_score(100.0)
                );
            }
        }
        results
    }
    async fn execute(&self, action: &Action, _context: &PluginContext) -> anyhow::Result<()> {
        let mut clipboard = Clipboard::new()?;
        if let Action::CopyToClipboard(word) = action {
            let _ = clipboard.set_text(word)?;
            println!("Word copied to clipboard: {}", word);
        }
        Ok(())
    }
}