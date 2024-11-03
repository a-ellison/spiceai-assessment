use std::io::{self, Write};

use arrow::array::cast::as_string_array;
use futures::stream::StreamExt;
use spiceai::ClientBuilder;

#[tokio::main]
async fn main() {
    let mut client = ClientBuilder::new().build().await.unwrap();

    let mut works_query = client
        .query("SELECT workid, title FROM works;")
        .await
        .expect("Failed to query works");

    // justify why it's ok to read a single batch
    if let Some(Ok(batch)) = works_query.next().await {
        let col_workid = as_string_array(batch.column(0));
        let col_title = as_string_array(batch.column(1));
        // TODO: maybe unnecessary to convert to vector
        let workids: Vec<&str> = col_workid.iter().map(|x| x.unwrap()).collect();
        let titles: Vec<&str> = col_title.iter().map(|x| x.unwrap()).collect();

        println!("Choose a work:");
        for (i, t) in titles.iter().enumerate() {
            println!("({}) {}", i + 1, t);
        }

        loop {
            print!("> ");
            let _ = io::stdout().flush();

            let mut choice = String::new();

            io::stdin()
                .read_line(&mut choice)
                .expect("Failed to read line");

            let choice: usize = choice.trim().parse().expect("Please type a number!");
            if !(choice > 0 && choice <= titles.len()) {
                println!("Choice must be in the range 1..={}!", titles.len());
                continue;
            }

            let workid = workids[choice - 1];

            let mut character_query = client.query(format!("SELECT charid, charname, works, speechcount FROM characters WHERE works LIKE '%{}%' ORDER BY speechcount LIMIT 10", workid).as_str()).await.expect("Failed to query characters");

            let batch = character_query.next().await.unwrap().unwrap();
            let charnames = as_string_array(batch.column(1));
            let charnames = charnames
                .iter()
                .map(|x| x.unwrap())
                .collect::<Vec<&str>>()
                .join("\n");
            println!(
                "The most popular characters in \"{}\" are:",
                titles[choice - 1]
            );
            println!("{}", charnames);
        }
    }
}
