use std::io;
use std::io::*;
use serde::{Deserialize, Serialize};
use tokio;
use colored::*;
use reqwest::{self, header::CONTENT_TYPE};
use serde_json::json;

const HELP_STR: &str = "
Commands List\n
    list: Lists all drops
    usage: list\n
    create: Creates drop
    usage: create <NAME> <PARAM> <SECRET> <TYPE> <STOCK>\n
    delete: Deletes specific drop
    usage: delete <NAME>\n
    view: Shows specific drop details
    usage: view <NAME>\n
    edit: Edits specific drop details
    usage: edit <NAME> <FIELD> <VALUE>
";

// ;)
const DROP_LISTS_ENDPOINT: &str = "http://localhost:3000/";
const DROP_EDIT_ENDPOINT: &str = "http://localhost:3000/";
const DROP_DELETE_ENDPOINT: &str = "http://localhost:3000/";
const DROP_CREATE_ENDPOINT: &str = "http://localhost:3000/";

const SECRET_HEADER_NAME: &str = "header";
const SECRET_HEADER_VALUE: &str = "value";

type ListsRes = Vec<DropS>;

#[derive(Debug, Serialize, Deserialize)]
struct DropS {
    name: String,
    param: String,
    secret: String,
    #[serde(rename = "type")]
    response_type: String,
    stock: i32,
    purchased: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct DropCreate {
    name: String,
    param: String,
    secret: String,
    #[serde(rename = "type")]
    response_type: String,
    stock: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct DropEdit {
    name: String,
    argument: String,
    value: ::serde_json::Value
}

#[derive(Debug, Serialize, Deserialize)]
struct DropEditRes {
    success: bool,
    message: DropS,
}

#[derive(Debug, Serialize, Deserialize)]
struct Success {
    success: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct Name {
    name: String,
}

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();

    println!("hello kappa!");

    loop {
        print!("{}", "k => ".purple());

        io::stdout().flush().expect("Error flushing");

        let mut string = String::new();

        io::stdin()
            .read_line(&mut string)
            .expect("Error parsing line");

        println!("");

        string.pop();

        let args: Vec<&str> = string.split(" ").collect();

        process_command(args, &client).await;
    }
}

async fn process_command(args: Vec<&str>, client: &reqwest::Client) {
    match args[0] {
        "help" => println!("{}", &HELP_STR),
        "list" => list_drops(&client).await,
        "view" => process_view_args(args, &client).await,
        "edit" => process_edit_args(args, &client).await,
        "delete" => process_delete_args(args, &client).await,
        "create" => process_create_args(args, &client).await,
        _ => (),
    };
}

async fn process_view_args(args: Vec<&str>, client: &reqwest::Client) {
    match args.get(1) {
        Some(_) => view_drop(&args[1], &client).await,
        None => return println!("View command requires 1 argument, 0 were provided\n"),
    };
}

async fn process_edit_args(mut args: Vec<&str>, client: &reqwest::Client) {
    args.remove(0);

    match args[1] {
        "name" | "param" | "secret" => match args.get(2) {
            Some(_) => (),
            None => return println!("Edit command requires 2 arguments, 1 was provided\n"),
        },
        "type" => match args[2] {
            "initial-lifetime" | "paid-lifetime" | "initial-renewal" | "paid-renewal" => (),
            _ => return println!("Type argument value must be a valid drop type\n"),
        },
        "stock" => match args[2].parse::<i32>() {
            Ok(_) => (),
            Err(_) => return println!("Stock argument value must be an integer\n"),
        },
        _ => return println!("Edit command argument 1 must be a valid argument (name, param, secret, type, stock)\n"),
    };

    edit_drop(args, &client).await;
}

async fn process_delete_args(args: Vec<&str>, client: &reqwest::Client) {
    match args.get(1) {
        Some(_) => delete_drop(&args[1], &client).await,
        None => return println!("Delete command requires 1 argument, 0 were provided\n"),
    };
}

async fn process_create_args(mut args: Vec<&str>, client: &reqwest::Client) {
    args.remove(0);

    if args.len() != 5 {
        return println!("Create command requires 4 arguments, {} were provided\n", args.len());
    }

    match args[3] {
        "initial-lifetime" | "paid-lifetime" | "initial-renewal" | "paid-renewal" => (),
        _ => return println!("Type argument value must be a valid drop type\n"),
    }

    match args[4].parse::<i32>() {
        Ok(_) => (),
        Err(_) => return println!("Stock argument value must be an integer\n"),
    }
    
    create_drop(args, &client).await
}

async fn list_drops(client: &reqwest::Client) {
    let url = format!("{}?all_drops=true", &DROP_LISTS_ENDPOINT);

    let res = client
        .get(&url)
        .header(SECRET_HEADER_NAME, SECRET_HEADER_VALUE)
        .send()
        .await;

    let body: ListsRes = match res {
        Ok(r) => r.json().await.unwrap(),
        Err(e) => return println!("Error while getting lists:\n{:?}\n", e),
    };

    if body.len() == 0 {
        return println!("There are currently no drops\n");
    }

    for drop in body.iter() {
        println!("Name: {}", drop.name);
        println!("Parameter: {}", drop.param);
        println!("Secret token: {}", drop.secret);
        println!("Type: {}", drop.response_type);
        println!("Stock: {}", drop.stock);
        println!("Purchased: {}\n", drop.purchased);
    }
}

async fn view_drop(dropname: &str, client: &reqwest::Client) {
    let url = format!("{}?drop={}", &DROP_LISTS_ENDPOINT, &dropname);

    let res = client
        .get(&url)
        .header(SECRET_HEADER_NAME, SECRET_HEADER_VALUE)
        .send()
        .await;

    let body: DropS = match res {
        Ok(r) => match r.json().await {
            Ok(b) => b,
            Err(_) => return println!("There are no drops named: {}\n", &dropname),
        },
        Err(e) => return println!("Error while getting list:\n{:?}\n", e),
    };

    println!("Name: {}", body.name);
    println!("Parameter: {}", body.param);
    println!("Secret token: {}", body.secret);
    println!("Type: {}", body.response_type);
    println!("Stock: {}", body.stock);
    println!("Purchased: {}\n", body.purchased);
}

async fn edit_drop(args: Vec<&str>, client: &reqwest::Client) {
    let body= match args[1] {
        "name" | "param" | "secret" | "type" => DropEdit {
            name: String::from(args[0]),
            argument: String::from(args[1]),
            value: json!(String::from(args[2])),
        },
        "stock" => DropEdit {
            name: String::from(args[0]),
            argument: String::from(args[1]),
            value: json!(args[2].parse::<i32>().unwrap()),
        },
        _ => return,
    };

    let res = client
        .post(DROP_EDIT_ENDPOINT)
        .header(CONTENT_TYPE, "application/json")
        .header(SECRET_HEADER_NAME, SECRET_HEADER_VALUE)
        .json(&body)
        .send()
        .await;

    let body: Success = match res {
        Ok(r) => match r.json().await {
            Ok(b) => b,
            Err(_) => return println!("There are no drops named: {}\n", &args[0]),
        },
        Err(e) => return println!("Error while editing drop:\n{:?}\n", e),
    };

    if body.success {
        return println!("Drop: {} has been edited\n", &args[0]);
    }
        
    println!("There are no drops named: {}\n", &args[0])
}

async fn delete_drop(name_string: &str, client: &reqwest::Client) {
    let body = Name {
        name: String::from(name_string),
    };

    let res = client
        .post(DROP_DELETE_ENDPOINT)
        .header(CONTENT_TYPE, "application/json")
        .header(SECRET_HEADER_NAME, SECRET_HEADER_VALUE)
        .json(&body)
        .send()
        .await;

    let body: Success = match res {
        Ok(r) => match r.json().await {
            Ok(b) => b,
            Err(_) => return println!("There are no drops named: {}\n", &name_string),
        },
        Err(e) => return println!("Error while deleting drop:\n{:?}\n", e),
    };

    if body.success {
        return println!("Drop: {} has been deleted\n", &name_string);
    }
    
    println!("There are no drops named: {}\n", &name_string)
}

async fn create_drop(args: Vec<&str>, client: &reqwest::Client) {
    let body: DropCreate = DropCreate {
        name: String::from(args[0]),
        param: String::from(args[1]),
        secret: String::from(args[2]),
        response_type: String::from(args[3]),
        stock: args[4].parse::<i32>().unwrap(),
    };

    let res = client
        .post(DROP_CREATE_ENDPOINT)
        .header(CONTENT_TYPE, "application/json")
        .header(SECRET_HEADER_NAME, SECRET_HEADER_VALUE)
        .json(&body)
        .send()
        .await;

    let body: Success = match res {
        Ok(r) => match r.json().await {
            Ok(b) => b,
            Err(_) => return println!("There was an error creating the drop\n"),
        },
        Err(e) => return println!("Error while creating drop:\n{:?}\n", e),
    };

    if body.success {
        return println!("Drop: {} has been created\n", &args[1]);
    }
    
    println!("There was an error creating the drop\n");
}