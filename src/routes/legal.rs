use std::fmt::{self};

use crate::templates::HtmlTemplate;

use askama::Template;
use axum::extract::Path;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};

pub struct LegalPage {
    pub title: &'static str,
    pub legal_type: LegalType,
}

const LEGAL_PAGES: [LegalPage; 2] = [
    LegalPage {
        title: "Terms of use",
        legal_type: LegalType::Terms,
    },
    LegalPage {
        title: "Privacy policy",
        legal_type: LegalType::Privacy,
    },
];

#[derive(Serialize, Deserialize, PartialEq, Eq)]
pub enum LegalType {
    #[serde(rename = "terms")]
    Terms,
    #[serde(rename = "privacy")]
    Privacy,
}

impl fmt::Display for LegalType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = serde_json::to_value(self)
            .ok()
            .and_then(|v| v.as_str().map(|s| s.to_owned()))
            .unwrap_or_default();
        write!(f, "{}", s)
    }
}

#[derive(Template)]
#[template(path = "legal.html")]
struct LegalTemplate<'a> {
    current_page: &'a LegalPage,
}

pub async fn get_legal(Path(legal_type): Path<LegalType>) -> impl IntoResponse {
    let current_page = &LEGAL_PAGES
        .iter()
        .find(|page| page.legal_type == legal_type)
        .unwrap_or(LEGAL_PAGES.first().unwrap());

    let template = LegalTemplate { current_page };
    let html_template = HtmlTemplate(template);
    let response = html_template.into_response();
    response
}
