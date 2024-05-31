use std::collections::HashMap;

use crate::tree::expression::ExpressionVariable;

pub struct ScopeStack {
    pub locals: HashMap<ExpressionVariable, usize>,
    stack: Vec<Scope>,
}

impl ScopeStack {
    pub fn new() -> Self {
        ScopeStack {
            stack: Vec::with_capacity(20),
            locals: HashMap::default(),
        }
    }

    /// Mark a variable as existing but "not ready yet"
    pub fn declare(&mut self, name: String) {
        if let Some(scope) = self.stack.last_mut() {
            scope.insert(name, false);
        }
    }

    /// Mark a variable as ready
    pub fn define(&mut self, name: String) {
        if let Some(scope) = self.stack.last_mut() {
            scope.insert(name, true);
        }
    }

    /// Checks if a variable has been declared but not defined in it's local scope
    pub fn is_locally_declared(&self, name: &String) -> bool {
        if let Some(map) = self.stack.last() {
            match map.get(name) {
                Some(defined) => !defined,
                None => false,
            }
        } else {
            false
        }
    }

    pub fn begin_scope(&mut self) {
        let scope = Scope::default();

        self.stack.push(scope);
    }

    pub fn end_scope(&mut self) -> Option<Scope> {
        self.stack.pop()
    }

    pub fn encode_resolved_variable(&mut self, variable: ExpressionVariable) {
        // 0 depth means local-est scope
        for (depth, scope) in Iterator::zip(0..self.stack.len(), self.stack.iter().rev()) {
            if scope.contains_key(&variable.identifier_name) {
                self.locals.insert(variable.clone(), depth);
            }
        }
    }
}

type Scope = HashMap<String, bool>;
