
struct User{
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}


struct Color(i32, i32, i32);

struct UnitLikeTest;

fn main() {
    let u1 = User{
        active: true,
        username: String::from("leonardo borges"),
        email:String::from("leocborgess@gail.com"),
        sign_in_count:0,
    };
    let u2 = User{
        username:String::from("kmelo da silva"),
        ..u1
    };

    //u = build_user(username:"leo", email:"leo@leo.com");
    //println!("{u}");
}

fn build_user(username: String, email:String) -> User{
    User{
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
