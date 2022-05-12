fn main() {
    let num: u8 = 255; // 0~255 2^8

    let _x = 2.0; // f64
    let _y:f32 = 3.0; // f32

    // 数值操作
    let _sum = 5+10;
    let _difference = 95.5 - 4.3;
    let _product = 4*30;
    let _quotient = 56.7/32.2;
    let _reminder = 54%5;
    
    println!("result {}",num);


    // Tuple
    let tup: (i32,f64,u8) = (-500,6.4,1);
    println!("{},{},{}", tup.0, tup.1, tup.2);

    let (x,y,z) = tup;
    println!("{},{},{}", x, y, z);

    // Array
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
    ];
    let _first = months[0];
    let _second = months[1];


    let _x = plus_five(6);


    let s1 = String::from("hell world");
    let s2 = s1;
    println!("{}",s2);
}

fn plus_five(_x:i32) ->i32{
    3
}


