use crate::scheme::{parse_with_policy, Policy, Uri, UriError, VanityCandidate};
use std::collections::HashMap;

pub type Parser = Box<dyn Fn(&str) -> Result<Uri, UriError> + Send + Sync>;
pub type Completer = Box<dyn Fn(&str) -> Result<Vec<String>, UriError> + Send + Sync>;

pub struct TypeRegistration {
    pub name: String,
    pub parser: Option<Parser>,
    pub completer: Option<Completer>,
}

#[derive(Default)]
pub struct Registry {
    types: HashMap<String, TypeRegistration>,
    policy: Policy,
}

impl Registry {
    pub fn new() -> Self {
        Self::with_policy(Policy::default())
    }

    pub fn with_policy(policy: Policy) -> Self {
        Self {
            types: HashMap::new(),
            policy,
        }
    }

    pub fn register(&mut self, reg: TypeRegistration) -> Result<(), UriError> {
        if reg.name.is_empty() {
            return Err(UriError::InvalidInput(
                "registration name is required".to_string(),
            ));
        }
        if self.types.contains_key(&reg.name) {
            return Err(UriError::InvalidInput(format!(
                "type {:?} already registered",
                reg.name
            )));
        }
        self.types.insert(reg.name.clone(), reg);
        Ok(())
    }

    pub fn parse(&self, input: &str) -> Result<Uri, UriError> {
        let parsed = parse_with_policy(input, &self.policy)?;
        let reg = self
            .types
            .get(&parsed.scheme)
            .ok_or_else(|| UriError::InvalidInput(format!("unknown type {:?}", parsed.scheme)))?;
        if let Some(parser) = &reg.parser {
            return parser(input);
        }
        Ok(parsed)
    }

    pub fn complete_vanity(&self, input: &str) -> Vec<VanityCandidate> {
        self.policy.vanity_candidates(input)
    }

    pub fn complete(&self, type_name: &str, prefix: &str) -> Result<Option<Vec<String>>, UriError> {
        let reg = self
            .types
            .get(type_name)
            .ok_or_else(|| UriError::InvalidInput(format!("unknown type {type_name:?}")))?;
        match &reg.completer {
            Some(completer) => completer(prefix).map(Some),
            None => Ok(None),
        }
    }

    pub fn types(&self) -> Vec<String> {
        let mut types: Vec<_> = self.types.keys().cloned().collect();
        types.sort();
        types
    }
}

pub struct CompletionResult {
    pub suggestions: Vec<String>,
}

pub fn complete_with_scheme(
    reg: &Registry,
    type_name: &str,
    to_complete: &str,
) -> Result<CompletionResult, UriError> {
    let (scheme, prefix) = to_complete.split_once("://").unwrap_or(("", to_complete));

    if !scheme.is_empty() {
        let candidates = reg.complete_vanity(to_complete);
        if candidates.len() > 1 {
            return Ok(CompletionResult {
                suggestions: candidates
                    .into_iter()
                    .map(|candidate| format!("{}\tcanonical: {}", candidate.from, candidate.to))
                    .collect(),
            });
        }
    }

    let suggestions = reg.complete(type_name, prefix)?.unwrap_or_default();
    let suggestions = if scheme.is_empty() {
        suggestions
    } else {
        suggestions
            .into_iter()
            .map(|suggestion| format!("{scheme}://{suggestion}"))
            .collect()
    };
    Ok(CompletionResult { suggestions })
}
