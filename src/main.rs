


fn send(client: reqwest::blocking::Client) {

    let res = client.post("http://localhost:8080")
        .body("simen")
        .send();
    
    println!("{res:?}")


}

fn main() {

    let client = reqwest::blocking::Client::new();
    
    send(client);

}