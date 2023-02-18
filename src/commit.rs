pub struct Commit {
    pub regex: String,
}

impl Commit {
    pub fn builder() -> CommitBuilder {
        CommitBuilder::new()
    }
}

pub struct CommitBuilder {
    regex: String,
}

impl CommitBuilder {
    pub fn new() -> Self {
        Self {
            regex: String::from('^'),
        }
    }

    pub fn build(mut self) -> Commit {
        self.regex.push('$');
        Commit { regex: self.regex }
    }

    pub fn optionnal(mut self) -> Self {
        self.regex.push('?');
        self
    }

    pub fn reverse(mut self) -> Self {
        let expression = CommitBuilder::group("revert: ");
        self.regex.push_str(&expression);
        self
    }

    pub fn types(mut self, types: Vec<&str>) -> Self {
        let expression = CommitBuilder::group(&types.join("|"));
        self.regex.push_str(&expression);
        self
    }

    pub fn scopes(mut self) -> Self {
        let expression = CommitBuilder::group(r"\(.+\)");
        self.regex.push_str(&expression);
        self
    }

    pub fn breaking(mut self) -> Self {
        self.regex.push('!');
        self
    }

    pub fn colon(mut self) -> Self {
        self.regex.push_str(": ");
        self
    }

    pub fn description(mut self, min: i32, max: i32) -> Self {
        let expression = format!(".{{{},{}}}", min, max);
        self.regex.push_str(&expression);
        self
    }

    fn group(expression: &str) -> String {
        format!("({})", expression)
    }
}
