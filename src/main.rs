use serde::Serialize;

#[derive(Serialize, Debug)]
struct Concert {
    event_name: String,
    event_date: String,
    event_location: String,
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

#[tokio::main]
async fn get_concerts(url: &str)  -> std::result::Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get(url)
        .await?
        .text()
        .await?;

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

   println!("{:#?}", concerts);
   Ok(())
}

fn main() {
     match get_concerts("http://concertsdallas.com") {
         Ok(_) => println!("Request was successful!"),
         Err(_) => println!("Request was unsuccessful!"),
     }
}
