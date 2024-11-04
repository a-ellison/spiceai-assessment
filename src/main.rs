use std::{
    io::{self, Write},
    iter::zip,
};
use arrow::{
    array::{
        cast::{as_primitive_array, as_string_array},
        Array,
    },
    datatypes::Int32Type,
};
use futures::stream::StreamExt;
use spiceai::{Client, ClientBuilder};

#[tokio::main]
async fn main() {
    let mut client = ClientBuilder::new()
        .build()
        .await
        .expect("Failed to build client");

    let mut works_query = client
        .query("SELECT workid, title FROM works;")
        .await
        .expect("Failed to query works");

    // assume result will fit in a single batch
    let batch = works_query
        .next()
        .await
        .expect("Failed to get batch from stream")
        .expect("Batch has error");
    let workids = as_string_array(batch.column(0));
    let titles = as_string_array(batch.column(1));

    println!("Choose a work:");
    for (i, t) in titles.iter().enumerate() {
        println!("({}) {}", i + 1, t.unwrap());
    }

    // very simple REPL
    loop {
        let choice = match read_choice(titles.len()) {
            Ok(x) => x - 1,
            Err(_) => continue,
        };

        let workid = workids.value(choice);

        println!("Information about \"{}\":\n", titles.value(choice));
        print_characters(&mut client, workid).await;
        println!();
        print_acts_count(&mut client, workid).await;
        println!();
        print_first_n_paragraphs(&mut client, workid, 5).await;
        print_last_n_paragraphs(&mut client, workid, 5).await;
    }
}

fn read_choice(n_titles: usize) -> Result<usize, ()> {
    print!("> ");
    let _ = io::stdout().flush();

    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    match choice.trim().parse() {
        Ok(x) => {
            if x > 0 && x <= n_titles {
                Ok(x)
            } else {
                println!("Choice must be in the range 1..={}!", n_titles);
                Err(())
            }
        }
        Err(_) => {
            println!("Please type a number!");
            Err(())
        }
    }
}

async fn print_characters(client: &mut Client, workid: &str) {
    let mut query = client.query(format!("SELECT charname, description FROM characters WHERE works LIKE '%{}%' ORDER BY speechcount LIMIT 10", workid).as_str()).await.expect("Failed to query characters");

    // assume result will fit in a single batch
    let batch = query
        .next()
        .await
        .expect("Failed to get batch from stream")
        .expect("Batch has error");
    let charnames = as_string_array(batch.column(0)).iter().map(|x| x.unwrap());
    let descriptions = as_string_array(batch.column(1)).iter().map(|x| x.unwrap());

    println!("The most frequent characters are:");
    for (c, d) in zip(charnames, descriptions) {
        match d {
            "" => {
                println!("{}", c);
            }
            _ => println!("{}: {}", c, d),
        }
    }
}

async fn print_acts_count(client: &mut Client, workid: &str) {
    let mut query = client
        .query(
            format!(
                "SELECT CAST(COUNT(DISTINCT section) as integer) FROM chapters WHERE workid = '{}'",
                workid
            )
            .as_str(),
        )
        .await
        .expect("Failed to query chapters");

    // assume result will fit in a single batch
    let batch = query
        .next()
        .await
        .expect("Failed to get batch from stream")
        .expect("Batch has error");

    let n_sections = as_primitive_array::<Int32Type>(batch.column(0)).value(0);

    println!("There are {} acts", n_sections);
}

async fn print_first_n_paragraphs(client: &mut Client, workid: &str, n: usize) {
    println!("The first {} paragraphs of the play:", n);
    print_paragraphs(client,
            format!(
                "SELECT charid, plaintext FROM paragraphs WHERE workid = '{}' ORDER BY paragraphnum ASC LIMIT {}",
                workid, n
            )
            .as_str(), 
        false
    ).await;
}

async fn print_paragraphs(client: &mut Client, sql_query: &str, reverse: bool) {
    let mut query = client
        .query(sql_query)
        .await
        .expect("Failed to query paragraphs");

    // assume result will fit in a single batch
    let batch = query
        .next()
        .await
        .expect("Failed to get batch from stream")
        .expect("Batch has error");

    let characters = as_string_array(batch.column(0)).iter().map(|x| x.unwrap());
    let paragraphs = as_string_array(batch.column(1))
        .iter()
        .map(|p| p.unwrap().replace("[p]", "").replace("\\n", "\n"));

    let zipped = zip(characters, paragraphs);

    let print_pair = |c: &str, p: &str| {
        println!("{}:", c);
        println!("{}", p.trim());
        println!();
    };

    if reverse {
        for (c, p) in zipped.rev() {
            print_pair(c, &p);
        }
    } else {
        for (c, p) in zipped {
            print_pair(c, &p);
        }
    }
}

async fn print_last_n_paragraphs(client: &mut Client, workid: &str, n: usize) {
    println!("The last {} paragraphs of the play:", n);
    print_paragraphs(client,
            format!(
                "SELECT charid, plaintext FROM paragraphs WHERE workid = '{}' ORDER BY paragraphnum DESC LIMIT {}",
                workid, n
            )
            .as_str(),
        true
    ).await;
}
