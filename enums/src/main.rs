#[derive(Debug)]
enum IPAddrKind{
    IPv4,
    IPv6,
}

#[derive(Debug)]
struct IpAddr{
    kind: IPAddrKind,
    address: String,
}

#[derive(Debug)]
enum IPAddrE{
    V4(String),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message{
    fn call(&self){
        println!("msg writed {:#?}", self)
    }
}

#[derive(Debug)]
enum USState{
    Alabama, 
    Alaska,
}

#[derive(Debug)]
enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter(USState),
}

fn coin_in_cents(coin:Coin)->u8{
    match coin{
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) =>{
            println!("state = {state:?}");
            25
    }
    }
}


fn main() {
    let ipV4Example = IpAddr{
        kind: IPAddrKind::IPv4,
        address: String::from("127.0.0.1"),
    };
    println!("ippppp {:#?}", ipV4Example);

    let IPV4 = IPAddrE::V4(String::from("0.0.0.0"));
    
    println!("ippppp {:#?}", IPV4);

    let msg = Message::Write(String::from("hello"));
    msg.call();
    let c = coin_in_cents(Coin::Quarter(USState::Alabama));
    println!("coin in cents {c}");

    let mut dice = 5;

    match dice{
        3 => add_hat(),
        5 => remove_hat(),
        other => move_player(other),
    }
   
    dice = 6;
    match dice{
        3 => add_hat(),
        5 => remove_hat(),
        _ => {println!("default")},
    }

    let config_max = Some(10u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }

    //let coin = Coin::Quarter(USState::Alabama);
    let coin = Coin::Dime;
    let mut count = 1;
    if let Coin::Quarter(state) = coin{
        println!("State quarter from {state:?}!");
    }else{
        count+=1;
    }
    println!("counttt {count}!");

    if coin == Coin::Quarter(USState::Alabama){
        println!("counttt {count}!")
    }

}

fn add_hat(){
    println!("add");
}

fn remove_hat(){
    println!("remove");
}

fn move_player(other: u8){
    println!("move {other}");
}
