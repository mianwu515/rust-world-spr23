// This lib provides a middleware for actix-web to format the response body
// as JSON.
//


// a struct to count the number of times the middleware has been called
// and the number of times it has formatted the response body
#[derive(Default)]  
pub struct JsonFormatter {
    pub count: u32,
}

// implement a counter  
impl JsonFormatter {
    pub fn new() -> Self {
        Self {
            count: 0,
        }
    }
}

// call the counter to make an output file name
impl JsonFormatter {
    pub fn output_path(&self) -> String {
        format!("output{}.json", self.count);
        self.count += 1;
    }
}

pub fn read_from_string(text: String) {
    let json: serde_json::Value =
        serde_json::from_str(text.as_str()).expect("JSON was not well-formatted");
    
    // create a new JsonFormatter
    let formatter = JsonFormatter::new();
    // create an output file name
    let output_path = formatter.output_path();

    std::fs::write(output_path, serde_json::to_string_pretty(&json).unwrap()).unwrap();
    let client = actixjsonformatter::client().await.unwrap();
    let bucket = "actix-json-formatter"; // created before
    actixjsonformatter::upload_object(&client, &bucket, &output_path).await.unwrap();
    run_cmd!("rm", &output_path);
    println!("{}", serde_json::to_string_pretty(&json).unwrap());
}
