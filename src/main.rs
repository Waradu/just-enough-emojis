fn main() {
    let args: Vec<String> = std::env::args().collect();
    let text = if args.len() > 1 {
        &args[1]
    } else {
        eprintln!("Usage: program_name \"Your text here\"");
        return;
    };

    let request_body = format!(r#"{{"text": "{}"}}"#, text);
    let url = "https://emojify.epilogue.team/api/convert";

    let response = ureq::post(url)
        .set("Content-Type", "application/json")
        .send_string(&request_body);

    match response {
        Ok(resp) => {
            if let Ok(json_body) = resp.into_string() {
                if let Some(result_start) = json_body.find("\"result\":\"") {
                    let result_end =
                        json_body[result_start + 10..].find('"').unwrap() + result_start + 10;
                    let result = &json_body[result_start + 10..result_end];
                    println!("{}", result);
                } else {
                    println!("Failed to find 'result' in the response.");
                }
            } else {
                println!("Failed to read response body.");
            }
        }
        Err(e) => {
            eprintln!("Request failed: {}", e);
        }
    }
}
