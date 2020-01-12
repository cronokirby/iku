use std::collections::HashMap;

#[derive(Debug)]
struct Scope<T> {
    // If a scope is nested, then it has access to its parent scopes,
    // otherwise it's detached from those scopes. When calling a function,
    // that function has a completely new scope, whereas an if statement has
    // access to the surrounding scope.
    nested: bool,
    // The variable definitions in this specific scope
    vars: HashMap<String, T>,
}

impl <T> Scope<T> {
    fn new(nested: bool) -> Self {
        Scope {
            nested,
            vars: HashMap::new(),
        }
    }

    // Try and access a variable in this specific scope
    fn get(&self, name: &str) -> Option<&T> {
        self.vars.get(name)
    }

    // Insert a new variable, or replace an existing one
    fn insert<S: Into<String>>(&mut self, name: S, value: T) {
        self.vars.insert(name.into(), value);
    }
}
/// This allows us to handle lexical scoping
///
/// This is useful for assigning types to variables, as well as assigned
/// values to them.
#[derive(Debug)]
pub struct Scopes<T> {
    scopes: Vec<Scope<T>>,
}

impl <T> Scopes<T> {
    pub fn new() -> Self {
        Scopes { scopes: Vec::new() }
    }

    // Enter a new scope
    pub fn enter(&mut self, nested: bool) {
        self.scopes.push(Scope::new(nested));
    }

    // Exit a scope
    pub fn exit(&mut self) {
        self.scopes.pop();
    }

    // Get the value of a variable
    pub fn get(&self, name: &str) -> Option<&T> {
        let mut res = None;
        for scope in self.scopes.iter().rev() {
            let found = scope.get(name);
            if found.is_some() {
                res = found;
                break;
            }
            if !scope.nested {
                break;
            }
        }
        res
    }

    // Get the value of a variable
    // Returns whether or not we managed to find a variable to set.
    pub fn set<S: Into<String>>(&mut self, name: S, value: T) -> bool {
        let name = name.into();
        for scope in self.scopes.iter_mut().rev() {
            if scope.get(&name).is_some() {
                scope.insert(name, value);
                return true;
            }
            if !scope.nested {
                break;
            }
        }
        false
    }

    // Create a new variable in the current scope
    // This panics if no scopes have been created
    pub fn create<S: Into<String>>(&mut self, name: S, value: T) {
        let name = name.into();
        self.scopes.last_mut().unwrap().insert(name, value);
    }
}
