fn main() {
    //break_example();
    //loop_with_label();
    //while_example();
    for_collection();
}

fn for_collection(){
    let a = [0,1,2,3,4];
    let mut index = 0;
    while index < 5 {
        println!("collection value = {}", a[index]);
        index +=1;
    }

    for element in a{
        println!("collection value = {element}");
        
    }

    for number in (1..4).rev(){
        println!("collection rev value = {number}");
    }

}

fn while_example() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn loop_with_label() {
    let mut count = 0;
    'count_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'count_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn break_example() {
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("counter value {result}");
}
