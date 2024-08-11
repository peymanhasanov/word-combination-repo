use itertools::Itertools;
use serde::Deserialize;
use warp::Filter;
use std::time::Instant;

#[derive(Deserialize)]
struct InputData {
    words: Vec<String>,
}

#[tokio::main]
async fn main() {
    // CORS ayarları
    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["POST"])
        .allow_headers(vec!["Content-Type"]);

    // API endpoint
    let generate = warp::path("generate")
        .and(warp::post())
        .and(warp::body::json())
        .map(|input_data: InputData| {
            let words = input_data.words;
            
            // Başlangıç zamanını kaydet
            let start_time = Instant::now();
            
            // Permütasyonları hesapla
            let permutations = words.iter().permutations(words.len());
            let mut result = Vec::new();
            
            for perm in permutations {
                result.push(perm.iter().map(|s| s.as_str()).collect::<Vec<&str>>().join(" ; "));
            }

            // Geçen süreyi hesapla
            let duration = start_time.elapsed();
            result.push(format!("\nHesaplama süresi: {:?}", duration));

            warp::reply::with_header(result.join("\n"), "Content-Type", "text/plain")
        })
        .with(cors);

    // Sunucu başlatma
    warp::serve(generate).run(([127, 0, 0, 1], 3030)).await;
}
