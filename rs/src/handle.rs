use std::error::Error;
use std::fmt;
use std::io::Read;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Language {
    Go,
    Ts,
    Python,
    Rust,
    Php,
    Other(String),
}

impl Language {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Go => "go",
            Self::Ts => "ts",
            Self::Python => "py",
            Self::Rust => "rs",
            Self::Php => "php",
            Self::Other(value) => value,
        }
    }
}

impl From<&str> for Language {
    fn from(value: &str) -> Self {
        match value {
            "go" => Self::Go,
            "ts" => Self::Ts,
            "py" => Self::Python,
            "rs" => Self::Rust,
            "php" => Self::Php,
            other => Self::Other(other.to_string()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HandlerSpec {
    pub vendor: String,
    pub app: String,
    pub instance: String,
    pub language: Language,
    pub scheme: String,
    pub version: String,
    pub channel: String,
    pub app_path: String,
    pub display_name: String,
}

impl HandlerSpec {
    pub fn handler_id(&self) -> Result<String, HandlerError> {
        self.validate()?;
        let mut parts = vec![self.vendor.as_str(), self.app.as_str()];
        if !self.instance.is_empty() {
            parts.push(self.instance.as_str());
        }
        parts.push(self.language.as_str());
        parts.push(self.scheme.as_str());
        Ok(parts.join("."))
    }

    pub fn validate(&self) -> Result<(), HandlerError> {
        for (field, value) in [
            ("vendor", self.vendor.as_str()),
            ("app", self.app.as_str()),
            ("language", self.language.as_str()),
            ("scheme", self.scheme.as_str()),
            ("app_path", self.app_path.as_str()),
        ] {
            if value.is_empty() {
                return Err(HandlerError::MissingField(field));
            }
        }

        match self.language {
            Language::Go | Language::Ts | Language::Python | Language::Rust | Language::Php => {}
            Language::Other(_) => {
                return Err(HandlerError::UnsupportedLanguage(
                    self.language.as_str().to_string(),
                ))
            }
        }

        for (field, value) in [
            ("vendor", self.vendor.as_str()),
            ("app", self.app.as_str()),
            ("instance", self.instance.as_str()),
            ("language", self.language.as_str()),
            ("scheme", self.scheme.as_str()),
        ] {
            if value.contains('/') || value.contains('\\') {
                return Err(HandlerError::PathSeparator(field));
            }
        }
        Ok(())
    }

    fn display_name(&self) -> String {
        if self.display_name.is_empty() {
            self.handler_id().unwrap_or_else(|_| self.app.clone())
        } else {
            self.display_name.clone()
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HandlerError {
    MissingField(&'static str),
    UnsupportedLanguage(String),
    PathSeparator(&'static str),
    UnknownPlatform(String),
    Io(String),
}

impl fmt::Display for HandlerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingField(field) => write!(f, "generate: {field} must not be empty"),
            Self::UnsupportedLanguage(language) => {
                write!(f, "generate: unsupported language {language:?}")
            }
            Self::PathSeparator(field) => {
                write!(f, "generate: {field} must not contain path separators")
            }
            Self::UnknownPlatform(platform) => write!(f, "generate: unknown platform {platform:?}"),
            Self::Io(err) => write!(f, "generate: io: {err}"),
        }
    }
}

impl Error for HandlerError {}

pub fn snippet(platform: &str, spec: &HandlerSpec) -> Result<String, HandlerError> {
    match platform {
        "macos" | "ios" => plist_snippet(spec),
        "linux" => desktop_file(spec),
        "windows" => windows_reg_snippet(spec),
        other => Err(HandlerError::UnknownPlatform(other.to_string())),
    }
}

pub fn desktop_file(spec: &HandlerSpec) -> Result<String, HandlerError> {
    let id = spec.handler_id()?;
    Ok(format!(
        "[Desktop Entry]\nType=Application\nName={}\nExec={} %u\nMimeType=x-scheme-handler/{};\nNoDisplay=true\nX-Hop-Handler-ID={}\n",
        spec.display_name(), spec.app_path, spec.scheme, id
    ))
}

pub fn desktop_filename(spec: &HandlerSpec) -> Result<String, HandlerError> {
    Ok(format!("{}.desktop", spec.handler_id()?))
}

pub fn plist_snippet(spec: &HandlerSpec) -> Result<String, HandlerError> {
    let id = spec.handler_id()?;
    Ok(format!(
        "<key>CFBundleURLTypes</key>\n<array>\n\t<dict>\n\t\t<key>CFBundleURLName</key>\n\t\t<string>{}</string>\n\t\t<key>CFBundleURLSchemes</key>\n\t\t<array>\n\t\t\t<string>{}</string>\n\t\t</array>\n\t</dict>\n</array>",
        id, spec.scheme
    ))
}

pub fn patch_plist<R: Read>(mut reader: R, spec: &HandlerSpec) -> Result<String, HandlerError> {
    spec.validate()?;
    let mut src = String::new();
    reader
        .read_to_string(&mut src)
        .map_err(|err| HandlerError::Io(err.to_string()))?;
    let snippet = plist_snippet(spec)?;
    if src.contains("</dict>\n</plist>") {
        return Ok(src.replacen("</dict>\n</plist>", &(snippet + "\n</dict>\n</plist>"), 1));
    }
    Ok(src.replacen("</dict></plist>", &(snippet + "\n</dict></plist>"), 1))
}

pub fn windows_reg_snippet(spec: &HandlerSpec) -> Result<String, HandlerError> {
    let id = spec.handler_id()?;
    let display_name = spec.display_name();
    Ok(format!(
        "Windows Registry Editor Version 5.00\r\n\r\n[HKEY_CURRENT_USER\\Software\\Classes\\{}]\r\n@=\"URL:{} Protocol\"\r\n\"URL Protocol\"=\"\"\r\n\"FriendlyTypeName\"=\"{}\"\r\n\"HopHandlerID\"=\"{}\"\r\n\r\n[HKEY_CURRENT_USER\\Software\\Classes\\{}\\shell\\open\\command]\r\n@=\"\\\"{}\\\" \\\"%1\\\"\"\r\n",
        spec.scheme, display_name, display_name, id, spec.scheme, spec.app_path
    ))
}
