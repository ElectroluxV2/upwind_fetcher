use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegattaOverview {
    pub regatta: Regatta,
    pub tables: Vec<Table>
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Venue {
    pub name: String,
    pub latitude: f64,
    pub longitude: f64
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Table {
    pub id: String,
    pub name: String,
    pub fetch_url: String
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Regatta {
    pub id: String,
    pub description: String,
    pub name: String,
    pub start_date: u64,
    pub end_date: u64,
    pub venue: Venue
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResults {
    pub form: Form,
    pub results: Results
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmptySearchResults {
    pub form: Form,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Form {
    pub children: FormChildren
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FormChildren {
    pub dinghies: Dinghies
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dinghies {
    pub options: DinghiesOptions
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DinghiesOptions {
    pub value: Vec<String>,
    pub full_name: String,
    pub choices: Vec<DinghiesChoice>
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DinghiesChoice {
    pub label: String,
    pub value: String
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Results {
    pub items: Vec<ResultsItem>,
    pub pagination: ResultsPagination
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResultsItem {
    pub id: String,
    pub name: String,
    pub start_date: u64,
    pub end_date: u64,
    pub dinghies: Vec<String>,
    pub url: String
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResultsPagination {
    pub current_page: u64,
    pub per_page: u64,
    pub total_items: u64,
    pub total_pages: u64
}
