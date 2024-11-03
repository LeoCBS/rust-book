use std::fmt::Display;

fn main() {
    let int_vec = vec![100, 150, 50, 85, 40];
    let largest = largest(&int_vec);
    println!("largest number in the list is {}", largest);
    let p = Point { x: 1, y: 2.0 };
    println!("point x generic {} / y {}", p.x(), p.y());

    let tweet = Tweet {
        username: String::from("leonardo.borges"),
        content: String::from("rust"),
        reply: false,
        retweet: false,
    };

    println!("tweet summarize {}", tweet.summarize());

    let new_article = NewArticle {
        headline: String::from("blabla"),
        location: String::from("brazil"),
        author: String::from("leonardo.borges"),
        content: String::from("my books"),
    };

    println!("tweet summarize {}", new_article.summarize());

    notify(&new_article);
    notify(&tweet);

    //lifetime
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {result}");
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn largest<T>(list: &[T]) -> &T
where
    T: std::cmp::PartialOrd,
{
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item
        }
    }
    largest
}

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &U {
        &self.y
    }
}

//traits chapter 10-02
pub trait Summary {
    //default behavior
    fn summarize(&self) -> String {
        format!("(Read more from author {})", self.summarize_author())
    }

    fn summarize_author(&self) -> String;
}

struct NewArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// trait bounds
//pub fn notify<T: Summary>(item1: &T, item2: &T) {

fn notify(item: &impl Summary) {
    println!("Breaking news {}", item.summarize())
}

fn notify2<T>(item: &T)
where
    T: Summary,
{
    println!("Breaking news {}", item.summarize())
}

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
