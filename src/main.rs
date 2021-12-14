use serde::Serialize;

#[derive(Serialize, Debug)]
struct Concert {
    event_name: String,
    event_date: String,
    event_location: String,
}

#[tokio::main]
async fn get_page_response(url: &str) -> Result<std::string::String, Box<dyn std::error::Error>> {
    let resp = reqwest::get(url)
        .await?
        .text()
        .await?;


    return Ok(resp);
}


fn serialize_events(events: Vec<Concert>) -> Vec<std::string::String> {
    let mut concerts = vec![];
    for event in events {
        let concert = serde_json::to_string(&event);
        match concert {
            Ok(concert) => concerts.push(concert),
            Err(_) => println!("Something went wrong!"),
            }
    }

    return concerts;
}

fn get_concerts(resp: &str)  -> Vec<std::string::String> {
    let table = table_extract::Table::find_by_id(&resp, "shTable").unwrap();

    let mut events = vec![];
    for row in &table {

        let event_name = row.as_slice().get(0).unwrap().to_string();
        let event_date = row.as_slice().get(1).unwrap().to_string();
        let event_location = row.as_slice().get(2).unwrap().to_string();

        let concert = Concert {
            event_name,
            event_date,
            event_location,
       };

        events.push(concert);
    }

   let concerts = serialize_events(events);

   return concerts; 
}

fn main() {
    let response = get_page_response("http://concertsdallas.com");
    let concerts = get_concerts(&response.unwrap());
   println!("{:#?}", concerts);
}
