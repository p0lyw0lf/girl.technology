use url::Url;

/// Checks that a given URL has a .well-known/girl.technology file, and returns the contents of
/// that file if it exists. Otherwise, returns a description of what went wrong.
async fn check_url(url: &str) -> Result<String, String> {
    let mut url = match Url::parse(url) {
        Ok(url) => url,
        Err(e) => return Err(format!("Error parsing url: {e}")),
    };

    match url.scheme() {
        "http" | "https" => {}
        scheme => return Err(format!("Invalid url scheme {scheme}")),
    };

    // Manipulate in a separate context, so that we drop the `mut path` before `url`
    {
        let mut path = match url.path_segments_mut() {
            Ok(path) => path,
            Err(e) => return Err(format!("Url doesn't have a path: {e:?}")),
        };

        path.extend([".well-known", "girl.technology"]);
    }

    let res = match reqwest::get(url.to_string()).await {
        Ok(res) => res,
        Err(e) => return Err(format!("Error fetching url: {e}")),
    };

    let text = match res.text().await {
        Ok(text) => text,
        Err(e) => return Err(format!("Error getting request body: {e}")),
    };

    Ok(text)
}
