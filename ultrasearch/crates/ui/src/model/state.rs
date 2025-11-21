use core_types::DocKey;
use gpui::{Context, Model, ModelContext, Task};
use ipc::{QueryExpr, SearchRequest, TermExpr, TermModifier};
use std::sync::Arc;
use crate::ipc::client::IpcClient;

const DEFAULT_MAX_ROWS: usize = 1000;

#[derive(Debug, Clone, PartialEq)]
pub struct ResultRow {
    pub doc_key: DocKey,
    pub name: String,
    pub path: String,
    pub ext: String,
    pub size: u64,
    pub modified_ts: i64,
    pub score: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BackendMode {
    #[default]
    MetadataOnly,
    Mixed,
    ContentOnly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortColumn {
    Name,
    Path,
    Size,
    Modified,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SortDirection {
    #[default]
    Asc,
    Desc,
}

#[derive(Debug, Default)]
pub struct SearchStatus {
    pub last_latency_ms: Option<u64>,
    pub shown: usize,
    pub total: usize,
    pub truncated: bool,
    pub backend_mode: BackendMode,
    pub connected: bool,
}

pub struct SearchAppModel {
    pub query: String,
    pub status: SearchStatus,
    pub results: Vec<ResultRow>,
    pub selected_index: Option<usize>,
    pub client: Arc<IpcClient>,
    pub sort_col: SortColumn,
    pub sort_dir: SortDirection,
    search_task: Option<Task<()>>,
}

impl SearchAppModel {
    pub fn init(cx: &mut Context) -> Model<Self> {
        cx.new_model(|_cx| Self {
            query: String::new(),
            status: SearchStatus::default(),
            results: Vec::with_capacity(DEFAULT_MAX_ROWS),
            selected_index: None,
            client: Arc::new(IpcClient::new()),
            sort_col: SortColumn::Name,
            sort_dir: SortDirection::Asc,
            search_task: None,
        })
    }

    pub fn set_query(&mut self, text: String, cx: &mut ModelContext<Self>) {
        self.query = text;
        cx.notify();
        
        // Debounce
        let task = cx.spawn(|this, mut cx| async move {
            let _ = cx.background_executor().timer(std::time::Duration::from_millis(150)).await;
            this.update(&mut cx, |model, cx| {
                model.perform_search(cx);
            }).ok();
        });
        self.search_task = Some(task);
    }

    pub fn set_backend_mode(&mut self, mode: BackendMode, cx: &mut ModelContext<Self>) {
        self.status.backend_mode = mode;
        cx.notify();
        self.perform_search(cx);
    }

    pub fn set_sort(&mut self, col: SortColumn, cx: &mut ModelContext<Self>) {
        if self.sort_col == col {
            self.sort_dir = match self.sort_dir {
                SortDirection::Asc => SortDirection::Desc,
                SortDirection::Desc => SortDirection::Asc,
            };
        } else {
            self.sort_col = col;
            self.sort_dir = SortDirection::Asc;
        }
        self.sort_results(cx);
    }

    pub fn sort_results(&mut self, cx: &mut ModelContext<Self>) {
        let dir = self.sort_dir;
        match self.sort_col {
            SortColumn::Name => self.results.sort_by(|a, b| {
                if dir == SortDirection::Asc { a.name.cmp(&b.name) } else { b.name.cmp(&a.name) }
            }),
            SortColumn::Path => self.results.sort_by(|a, b| {
                if dir == SortDirection::Asc { a.path.cmp(&b.path) } else { b.path.cmp(&a.path) }
            }),
            SortColumn::Size => self.results.sort_by(|a, b| {
                if dir == SortDirection::Asc { a.size.cmp(&b.size) } else { b.size.cmp(&a.size) }
            }),
            SortColumn::Modified => self.results.sort_by(|a, b| {
                if dir == SortDirection::Asc { a.modified_ts.cmp(&b.modified_ts) } else { b.modified_ts.cmp(&a.modified_ts) }
            }),
        }
        cx.notify();
    }

    pub fn perform_search(&mut self, cx: &mut ModelContext<Self>) {
        // Cancel previous task by dropping it (overwriting)
        // However, set_query already set a task (the timer).
        // We want the actual search logic to be the *new* task.
        // But set_query calls this *after* timer. 
        // If set_query is called again before timer fires, the old task is dropped, timer cancelled.
        // So the debounce logic in set_query covers the "wait before search".
        // But if we are *searching* (network), we also want to cancel that if a new query comes.
        // So set_query handles the debounce cancellation.
        // perform_search should set the *network* task.
        
        if self.query.is_empty() {
            self.update_results(vec![], 0, 0, cx);
            self.search_task = None;
            return;
        }

        let client = self.client.clone();
        let query_text = self.query.clone();
        
        let task = cx.spawn(move |model, mut cx| async move {
            // TODO: Parse query text into real AST (c00.7.3)
            // For now, just treat as prefix term on name
            let query = QueryExpr::Term(TermExpr {
                field: None,
                value: query_text,
                modifier: TermModifier::Prefix,
            });

            let req = SearchRequest {
                query,
                limit: DEFAULT_MAX_ROWS as u32,
                ..Default::default()
            };

            match client.search(req).await {
                Ok(resp) => {
                    model.update(&mut cx, |this, cx| {
                        let rows = resp.hits.into_iter().map(|h| ResultRow {
                            doc_key: h.key,
                            name: h.name.unwrap_or_default(),
                            path: h.path.unwrap_or_default(),
                            ext: h.ext.unwrap_or_default(),
                            size: h.size.unwrap_or_default(),
                            modified_ts: h.modified.unwrap_or_default(),
                            score: h.score,
                        }).collect();
                        
                        this.update_results(rows, resp.total as usize, resp.took_ms as u64, cx);
                        this.set_connected(true, cx);
                    }).ok();
                }
                Err(e) => {
                    tracing::error!("Search failed: {e}");
                    model.update(&mut cx, |this, cx| {
                        this.set_connected(false, cx);
                    }).ok();
                }
            }
        });
        self.search_task = Some(task);
    }

    pub fn update_results(
        &mut self, 
        rows: Vec<ResultRow>, 
        total: usize, 
        latency_ms: u64, 
        cx: &mut ModelContext<Self>
    ) {
        let truncated = rows.len() < total;
        self.status.last_latency_ms = Some(latency_ms);
        self.status.shown = rows.len();
        self.status.total = total;
        self.status.truncated = truncated;
        self.results = rows;
        // Re-apply sort
        self.sort_results(cx);
        // Reset selection if invalid
        if let Some(idx) = self.selected_index {
            if idx >= self.results.len() {
                self.selected_index = None;
            }
        }
        cx.notify();
    }

    pub fn set_connected(&mut self, connected: bool, cx: &mut ModelContext<Self>) {
        if self.status.connected != connected {
            self.status.connected = connected;
            cx.notify();
        }
    }

    pub fn select_index(&mut self, index: Option<usize>, cx: &mut ModelContext<Self>) {
        self.selected_index = index;
        cx.notify();
    }
    
    pub fn selected_row(&self) -> Option<&ResultRow> {
        self.selected_index.and_then(|i| self.results.get(i))
    }
}