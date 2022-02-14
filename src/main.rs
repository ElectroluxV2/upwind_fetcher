mod types;
use serde::de::DeserializeOwned;
use crate::types::{EmptySearchResults, ResultsPagination, SearchResults};

const OPEN_SKIFF_NAME_PATTERN: &str = "Skiff";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let empty_search_results = fetch::<EmptySearchResults>("https://www.upwind24.pl/search/regatta.json").await?;

    let open_skiff_choice = empty_search_results
        .form
        .children
        .dinghies
        .options
        .choices
        .into_iter()
        .find(|choice| choice.label.contains(OPEN_SKIFF_NAME_PATTERN))
        .expect("Cannot find O'pen Skiff ID");

    let dinghy_param_name = empty_search_results.form.children.dinghies.options.full_name;
    let dinghy_param_value = open_skiff_choice.value;
    let search_base_url = &format!("https://www.upwind24.pl/search/regatta.json?{dinghy_param_name}={dinghy_param_value}");
    let mut open_skiff_regattas = fetch::<SearchResults>(search_base_url).await?;

    loop {
        let ResultsPagination {current_page, total_pages, .. } = open_skiff_regattas.results.pagination;
        let next_page = current_page + 1;

        println!("Current page: {current_page}, next {next_page}, total: {total_pages}");

        if current_page >= total_pages {
            break;
        }

        open_skiff_regattas = fetch::<SearchResults>(&format!("{search_base_url}&page={next_page}")).await?;
    }


    // let results = fetch::<RegattaOverview>("https://www.upwind24.pl/puchar-pucka-2018/results.json").await?;
    //
    // println!("{:#?}", results);
    //
    // let url = results
    //     .tables
    //     .into_iter()
    //     .find(|table| table.name.contains(OPEN_SKIFF_NAME_PATTERN))
    //     .expect("Can not find O'pen Skiff results table")
    //     .fetch_url;
    //
    // println!("URL: {url}");
    //
    // let table = fetch::<RegattaOverview>(&url).await?;
    //
    // println!("{:#?}", table);

    Ok(())
}

async fn parse_regatta(url: &str) {

}

async fn fetch<T: DeserializeOwned>(url: &str) -> reqwest::Result<T> {
    reqwest::get(url).await?.json::<T>().await
}
