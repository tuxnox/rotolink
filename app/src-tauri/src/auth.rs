use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode , decode , Header , Validation , EncodingKey , DecodingKey , errors::Result , TokenData};
use std::time::{SystemTime , UNIX_EPOCH};
use std::fs;
use actix_web::{web , HttpResponse , Responder , post};
use rand::Rng;
use bcrypt::{hash, DEFAULT_COST , verify};



#[derive(Debug , Clone , Serialize , Deserialize)]
struct User {
    id : i32,
    username : String,
    password : String,
}


#[derive(Debug , Serialize , Deserialize)]
struct Claims {
    sub : String,
    exp : usize
}

#[derive(Deserialize)]
pub struct PairRequest {
    pub key : String,
    pub username : String , 
    pub password : String
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username : String,
    pub password : String,
}

pub fn load_key() -> String{
    let key  = match fs::read_to_string("Key.txt"){
        Ok(key) => key ,
        Err(_) => {
            let key = key_gen();
            key
        }
    };
    key
}
pub fn key_gen() -> String{
    let key = rand::rng().random_range(1000..9999);
    let key = key.to_string();
    let _ = key_save(key.clone());
    key
}
fn key_save(key : String){
    let _ = match fs::write("Key.txt" , key.clone()) {
        Ok(_) => {},
        Err(_) => {key_save(key);}
    };
}

fn create_jwt(username : &str , secret : &[u8]) -> Result<String> {
    let exp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() + 3600; 
    let claims = Claims {
        sub : username.to_string(),
        exp : exp as usize,
    };
    encode(&Header::default() , &claims , &EncodingKey::from_secret(secret) )
}

//fn verify_jwt(token : &str , secret : &[u8] ) -> Result<TokenData<Claims>> {
    //decode::<Claims>(token , &DecodingKey::from_secret(secret) , &Validation::default())
//}

fn load_credentials() -> Vec<User> {
    let file = fs::read_to_string("credentials.json").unwrap_or_else(|_| "[]".to_string());
    serde_json::from_str(&file).unwrap_or_else(|_| vec![])
}

fn save_credentials(users: &Vec<User>) {
    match fs::File::open("credentials.json"){
        Ok(_) => {},
        Err(_) => {let _ = fs::File::create("credentials.json");}
    }
    let json = serde_json::to_string(users).expect("Unable to convert to JSON");
    fs::write("credentials.json", json).expect("Unable to write file");
}

#[post("/register")]
pub async fn register(register_info : web::Json<PairRequest>) -> impl Responder {
    let key = load_key();
    let mut responce = HttpResponse::Unauthorized().body("Invalid pairing Key");
    if key == register_info.key.clone() {
        let mut users = load_credentials(); 
        let id = rand::rng().random_range(1..=130);
        let new_user = User {
        id,
        username: register_info.username.clone(),
        password: hash(register_info.password.clone(), DEFAULT_COST).unwrap(),
        };
    users.push(new_user);
    save_credentials(&users);
    responce = HttpResponse::Ok().body("User registered successfully");
    };
    let _ = key_gen();
    responce  
}

#[post("/login")]
pub async fn login(login_info : web::Json<LoginRequest>) -> impl Responder {
    let users = load_credentials();
    let mut result = HttpResponse::Unauthorized().body("Invalid credentials");
    for credentials in users.iter() {
        let username = credentials.username.clone();
        let password = credentials.password.clone().to_string();
        let id = credentials.id as u8;
        if login_info.username == username && verify(&login_info.password ,password.as_str()).unwrap() {
            let token = create_jwt(&login_info.username , &[id]).unwrap();
            result =  HttpResponse::Ok().json(token);
        }

    }
    result
}

