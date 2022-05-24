use libretranslate::{translate_url, Language};

fn language_mapping(s: &String) -> Language {
    let lang = match s.as_str() {
        "ENG" => Language::English,
        "CHT" => Language::Chinese,
        _ => Language::Detect,
    };

    lang
}

#[tokio::main]
pub async fn fy_handle(input: String, _source: String, _target: String) -> String {
    let source = language_mapping(&_source);
    let target = language_mapping(&_target);
    let url = "https://translate.argosopentech.com".to_string();

    println!("source:{}, target:{}.", _source, _target);

    let data = translate_url(source, target, input, url, None).await;

    let ret_str = match data {
        Ok(data) => data.output.to_string(),
        Err(error) => error.to_string(),
    };

    ret_str
}
