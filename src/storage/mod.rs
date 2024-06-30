pub mod csv;
pub mod json;

pub const ALLOWED_CONTENT_TYPES: [&str; 4] = [
    "text/html",
    "text/plain",
    "application/xhtml+xml",
    "application/xml"
];

pub fn is_allowed_content_type(content_type: &str) -> bool {
    ALLOWED_CONTENT_TYPES.iter().any(|&allowed| content_type.starts_with(allowed))
}
