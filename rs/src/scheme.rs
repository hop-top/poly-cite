use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use url::Url;

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Uri {
    pub scheme: String,
    pub namespace: String,
    pub id: String,
    pub query: String,
    pub fragment: String,
    pub original: String,
    pub action: String,
}

impl Uri {
    pub fn canonical(&self) -> String {
        let mut out = format!("{}://{}/{}", self.scheme, self.namespace, self.id);
        if !self.query.is_empty() {
            out.push('?');
            out.push_str(&self.query);
        }
        if !self.fragment.is_empty() {
            out.push('#');
            out.push_str(&self.fragment);
        }
        out
    }

    pub fn vanity(&self) -> String {
        if self.original.is_empty() {
            self.canonical()
        } else {
            self.original.clone()
        }
    }
}

impl fmt::Display for Uri {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.canonical())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Policy {
    pub default_namespace_segments: usize,
    pub scheme_namespace_segments: HashMap<String, usize>,
    pub vanity_aliases: Vec<VanityAlias>,
    pub action_routes: HashMap<String, ActionRoute>,
}

impl Default for Policy {
    fn default() -> Self {
        Self {
            default_namespace_segments: 1,
            scheme_namespace_segments: HashMap::from([
                ("task".to_string(), 2),
                ("doc".to_string(), 2),
                ("repo".to_string(), 1),
                ("tlc".to_string(), 2),
                ("task-dev".to_string(), 2),
                ("task-stress".to_string(), 2),
            ]),
            vanity_aliases: Vec::new(),
            action_routes: HashMap::new(),
        }
    }
}

impl Policy {
    pub fn namespace_segments(&self, scheme: &str) -> usize {
        self.scheme_namespace_segments
            .get(scheme)
            .copied()
            .or_else(|| {
                (self.default_namespace_segments != 0).then_some(self.default_namespace_segments)
            })
            .unwrap_or(1)
    }

    pub fn resolve_action(&self, uri: &Uri) -> Result<ResolvedAction, UriError> {
        if uri.action.is_empty() {
            return Err(UriError::ActionRequired);
        }
        let route = self
            .action_routes
            .get(&uri.action)
            .ok_or_else(|| UriError::UnknownAction(uri.action.clone()))?;
        if route.command.is_empty() {
            return Err(UriError::ActionRouteCommandRequired);
        }
        Ok(ResolvedAction {
            action: uri.action.clone(),
            command: expand_action_template(&route.command, uri),
            args: route
                .args
                .iter()
                .map(|arg| expand_action_template(arg, uri))
                .collect(),
        })
    }

    pub fn vanity_candidates(&self, input: &str) -> Vec<VanityCandidate> {
        let mut candidates: Vec<_> = self
            .vanity_aliases
            .iter()
            .filter_map(|alias| {
                let distance = levenshtein(input, &alias.from);
                within_fuzzy_threshold(input, &alias.from, distance).then(|| VanityCandidate {
                    from: alias.from.clone(),
                    to: alias.to.clone(),
                    distance,
                })
            })
            .collect();
        candidates.sort_by(|a, b| {
            a.distance
                .cmp(&b.distance)
                .then_with(|| a.from.cmp(&b.from))
        });
        candidates
    }

    fn resolve_vanity(
        &self,
        input: &str,
        options: ParseOptions,
    ) -> Result<(String, String), UriError> {
        let mut best: Option<&VanityAlias> = None;
        for alias in &self.vanity_aliases {
            if alias.from.is_empty() || alias.to.is_empty() {
                return Err(UriError::VanityAliasRequired);
            }
            if alias.to.contains("://") && Url::parse(&alias.to).is_err() {
                return Err(UriError::InvalidVanityTarget(alias.to.clone()));
            }
            let matched = input == alias.from
                || (alias.prefix && input.starts_with(&(alias.from.clone() + "/")));
            if matched && best.is_none_or(|current| alias.from.len() > current.from.len()) {
                best = Some(alias);
            }
        }

        if let Some(alias) = best {
            let mut target = alias.to.clone();
            if alias.prefix && alias.preserve_suffix && input.len() > alias.from.len() {
                target = format!(
                    "{}{}",
                    target.trim_end_matches('/'),
                    &input[alias.from.len()..]
                );
            }
            return Ok((target, input.to_string()));
        }

        if !options.strict {
            let candidates = self.vanity_candidates(input);
            if let Some(first) = candidates.first() {
                let best_distance = first.distance;
                let target = first.to.clone();
                let best: Vec<_> = candidates
                    .into_iter()
                    .take_while(|candidate| candidate.distance == best_distance)
                    .collect();
                if best.len() > 1 {
                    return Err(UriError::AmbiguousVanity(AmbiguousVanityError {
                        input: input.to_string(),
                        candidates: best,
                        as_json: options.json_ambiguity,
                    }));
                }
                return Ok((target, input.to_string()));
            }
        }

        Ok((input.to_string(), String::new()))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ParseOptions {
    pub strict: bool,
    pub json_ambiguity: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VanityAlias {
    pub from: String,
    pub to: String,
    #[serde(default)]
    pub prefix: bool,
    #[serde(default, rename = "preserveSuffix")]
    pub preserve_suffix: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActionRoute {
    pub command: String,
    #[serde(default)]
    pub args: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedAction {
    pub action: String,
    pub command: String,
    pub args: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VanityCandidate {
    pub from: String,
    pub to: String,
    pub distance: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AmbiguousVanityError {
    pub input: String,
    pub candidates: Vec<VanityCandidate>,
    pub as_json: bool,
}

impl fmt::Display for AmbiguousVanityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.as_json {
            let candidates = self
                .candidates
                .iter()
                .map(|candidate| {
                    format!(
                        r#"{{"from":"{}","to":"{}","distance":{}}}"#,
                        json_escape(&candidate.from),
                        json_escape(&candidate.to),
                        candidate.distance
                    )
                })
                .collect::<Vec<_>>()
                .join(",");
            return write!(
                f,
                r#"{{"input":"{}","candidates":[{}]}}"#,
                json_escape(&self.input),
                candidates
            );
        }
        let names = self
            .candidates
            .iter()
            .map(|candidate| candidate.from.as_str())
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "cite: ambiguous vanity alias {:?}: {}", self.input, names)
    }
}

impl Error for AmbiguousVanityError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UriError {
    EmptyInput,
    MissingScheme,
    EmptyNamespace,
    MissingId,
    InvalidInput(String),
    InvalidQuery(String),
    InvalidPathSegment(String),
    InvalidNamespaceSegmentCount,
    VanityAliasRequired,
    InvalidVanityTarget(String),
    AmbiguousVanity(AmbiguousVanityError),
    ConflictingActionQueryParameters,
    CmdAndVerbRequired,
    NameAndActionRequired,
    ActionRequired,
    UnknownAction(String),
    ActionRouteCommandRequired,
}

impl fmt::Display for UriError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyInput => f.write_str("cite: empty input"),
            Self::MissingScheme => f.write_str("cite: scheme is required"),
            Self::EmptyNamespace => f.write_str("cite: namespace is required"),
            Self::MissingId => f.write_str("cite: id is required"),
            Self::InvalidInput(err) => write!(f, "cite: invalid input: {err}"),
            Self::InvalidQuery(err) => write!(f, "cite: invalid query: {err}"),
            Self::InvalidPathSegment(err) => write!(f, "cite: invalid path segment: {err}"),
            Self::InvalidNamespaceSegmentCount => {
                f.write_str("cite: namespace segment count must be positive")
            }
            Self::VanityAliasRequired => f.write_str("cite: vanity alias from and to are required"),
            Self::InvalidVanityTarget(target) => write!(f, "cite: invalid vanity target: {target}"),
            Self::AmbiguousVanity(err) => err.fmt(f),
            Self::ConflictingActionQueryParameters => {
                f.write_str("cite: conflicting action query parameters")
            }
            Self::CmdAndVerbRequired => f.write_str("cite: cmd and verb must be provided together"),
            Self::NameAndActionRequired => {
                f.write_str("cite: name and action must be provided together")
            }
            Self::ActionRequired => f.write_str("cite: action is required"),
            Self::UnknownAction(action) => write!(f, "cite: unknown action {action:?}"),
            Self::ActionRouteCommandRequired => {
                f.write_str("cite: action route command is required")
            }
        }
    }
}

impl Error for UriError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::AmbiguousVanity(err) => Some(err),
            _ => None,
        }
    }
}

pub fn parse(input: &str) -> Result<Uri, UriError> {
    parse_with_policy_options(input, &Policy::default(), ParseOptions::default())
}

pub fn parse_with_policy(input: &str, policy: &Policy) -> Result<Uri, UriError> {
    parse_with_policy_options(input, policy, ParseOptions::default())
}

pub fn parse_with_policy_options(
    input: &str,
    policy: &Policy,
    options: ParseOptions,
) -> Result<Uri, UriError> {
    if input.is_empty() {
        return Err(UriError::EmptyInput);
    }

    let (parse_input, original) = policy.resolve_vanity(input, options)?;
    let parsed = Url::parse(&parse_input).map_err(|err| {
        if parse_input.contains("://") {
            UriError::InvalidInput(err.to_string())
        } else {
            UriError::MissingScheme
        }
    })?;

    if parsed.scheme().is_empty() {
        return Err(UriError::MissingScheme);
    }
    let host = parsed.host_str().ok_or(UriError::EmptyNamespace)?;
    if host.is_empty() {
        return Err(UriError::EmptyNamespace);
    }

    let mut segments = vec![host.to_string()];
    if let Some(path_segments) = parsed.path_segments() {
        for segment in path_segments.filter(|segment| !segment.is_empty()) {
            segments.push(segment.to_string());
        }
    }

    let namespace_segments = policy.namespace_segments(parsed.scheme());
    if namespace_segments == 0 {
        return Err(UriError::InvalidNamespaceSegmentCount);
    }
    if segments.len() <= namespace_segments {
        return Err(UriError::MissingId);
    }

    let namespace = segments[..namespace_segments].join("/");
    let id = segments[namespace_segments..].join("/");
    if namespace.is_empty() {
        return Err(UriError::EmptyNamespace);
    }
    if id.is_empty() {
        return Err(UriError::MissingId);
    }

    let query = parsed.query().unwrap_or_default().to_string();
    let action = action_from_query(&query)?;

    Ok(Uri {
        scheme: parsed.scheme().to_string(),
        namespace,
        id,
        query,
        fragment: parsed.fragment().unwrap_or_default().to_string(),
        original,
        action,
    })
}

fn action_from_query(raw_query: &str) -> Result<String, UriError> {
    let values = query_values(raw_query)?;
    let action = values.get("action").cloned().unwrap_or_default();
    let name = values.get("name").cloned().unwrap_or_default();
    let cmd = values.get("cmd").cloned().unwrap_or_default();
    let verb = values.get("verb").cloned().unwrap_or_default();

    let mut candidates = Vec::new();
    if !action.is_empty() && name.is_empty() {
        candidates.push(action.clone());
    }
    if !cmd.is_empty() || !verb.is_empty() {
        if cmd.is_empty() || verb.is_empty() {
            return Err(UriError::CmdAndVerbRequired);
        }
        candidates.push(format!("{cmd}.{verb}"));
    }
    if !name.is_empty() {
        if action.is_empty() {
            return Err(UriError::NameAndActionRequired);
        }
        candidates.push(format!("{name}.{action}"));
    }

    if candidates.is_empty() {
        return Ok(String::new());
    }
    if candidates
        .iter()
        .any(|candidate| candidate != &candidates[0])
    {
        return Err(UriError::ConflictingActionQueryParameters);
    }
    Ok(candidates.remove(0))
}

fn query_values(raw_query: &str) -> Result<HashMap<String, String>, UriError> {
    let mut values = HashMap::new();
    if raw_query.is_empty() {
        return Ok(values);
    }
    for pair in raw_query.split('&') {
        let (key, value) = pair.split_once('=').unwrap_or((pair, ""));
        values.insert(key.to_string(), value.to_string());
    }
    Ok(values)
}

fn expand_action_template(value: &str, uri: &Uri) -> String {
    value
        .replace("{scheme}", &uri.scheme)
        .replace("{namespace}", &uri.namespace)
        .replace("{id}", &uri.id)
        .replace("{query}", &uri.query)
        .replace("{fragment}", &uri.fragment)
}

fn within_fuzzy_threshold(input: &str, candidate: &str, distance: usize) -> bool {
    let longest = input.len().max(candidate.len());
    let threshold = (longest / 5).clamp(2, 8);
    distance <= threshold
}

fn levenshtein(a: &str, b: &str) -> usize {
    let ar: Vec<char> = a.chars().collect();
    let br: Vec<char> = b.chars().collect();
    if ar.is_empty() {
        return br.len();
    }
    if br.is_empty() {
        return ar.len();
    }

    let mut prev: Vec<usize> = (0..=br.len()).collect();
    let mut curr = vec![0; br.len() + 1];
    for (i, ac) in ar.iter().enumerate() {
        curr[0] = i + 1;
        for (j, bc) in br.iter().enumerate() {
            let cost = usize::from(ac != bc);
            curr[j + 1] = (curr[j] + 1).min(prev[j + 1] + 1).min(prev[j] + cost);
        }
        std::mem::swap(&mut prev, &mut curr);
    }
    prev[br.len()]
}

fn json_escape(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}
