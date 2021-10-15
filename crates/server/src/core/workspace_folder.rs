#[derive(Clone)]
pub struct WorkspaceFolder(pub lsp::WorkspaceFolder);

impl WorkspaceFolder {
    pub fn uri(&self) -> &lsp::Url {
        &self.0.uri
    }

    pub fn name(&self) -> &str {
        self.0.name.as_str()
    }
}

#[allow(unsafe_code)]
unsafe impl Send for WorkspaceFolder {
}

#[allow(unsafe_code)]
unsafe impl Sync for WorkspaceFolder {
}

impl Unpin for WorkspaceFolder {
}

impl std::fmt::Debug for WorkspaceFolder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::cmp::Eq for WorkspaceFolder {
}

impl std::cmp::Ord for WorkspaceFolder {
    fn cmp(&self, that: &Self) -> std::cmp::Ordering {
        let this = self.0.uri.as_str();
        let that = that.0.uri.as_str();
        std::cmp::Ord::cmp(this, that)
    }
}

impl std::cmp::PartialEq for WorkspaceFolder {
    fn eq(&self, that: &Self) -> bool {
        let this = self.0.uri.as_str();
        let that = that.0.uri.as_str();
        std::cmp::PartialEq::eq(this, that)
    }
}

impl std::cmp::PartialOrd for WorkspaceFolder {
    fn partial_cmp(&self, that: &Self) -> Option<std::cmp::Ordering> {
        let this = self.0.uri.as_str();
        let that = that.0.uri.as_str();
        std::cmp::PartialOrd::partial_cmp(this, that)
    }
}

impl std::hash::Hash for WorkspaceFolder {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.uri.hash(state)
    }
}
