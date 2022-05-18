use std::sync::Arc;

#[derive(Clone)]
pub struct GlobalRef {
    inner: Arc<GlobalRefGuard>,
}

struct GlobalRefGuard {}