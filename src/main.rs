fn main() {
    // This is the way we use comment
    /*
        The other way
    */
    println!("Hello, Rust!");

    let x = 45; // 선언
    let mut y = 60; // 변경 가능한 변수 선언

    println!("The value of x is {}", x);
    println!("The value of y is {}", y);
    // x = 60; rust에서는 이 짓이 불가

    y = 70;
    println!("The value of y is {}", y);

    // Data Types
    let z = 200; // i32라는 자료형이 자동으로 할당됨
    /*
        i8 : 8-bit assigned integer (unsigned : u8 이런식)
        i16 : 16-bit assigned integer
        i32 : 32-bit assigned integer
        i64 : 64-bit assigned integer
        i128
        isize : 64비트 OS이면 64비트로, 32비트면 32비트로  (unsigned : usize)
    */
    /*
        지정된 자료형에서 벗어나게 되면 panic이라는 에러가 발생한다.
        컴파일할 때 --release를 하게 되면 panic 체크를 안 하기 때문에 [u8이면 256 -> 0, 257 -> 1 요딴식으로 됨]
    */
    /*
        floating types
        f32, f64

        boolean types
        bool : true, false

        character types
        char
    */

    // 자료형 명시
    let foo: u8 = 255;
    let bar: f32 = 255.0;
    let baz: char = '🤔';
    let boolean: bool = true;


    // 조건문
    if foo < 254 {
        println!("foo is less than 255.")
    } else if foo == 255 {
        println!("foo is 255")
    } else {
        println!("unreachable")
    }


    // 반복문 loop? very simple
    let mut n = 0;
    loop {
        n += 1;

        if n == 3 {
            continue;
        }
        println!("The Value of n is {}", n);

        if n >= 6 {
            break;
        }
    }
    println!("The Value of n is {}", n);

    // 반복문 while
    let mut m = 0;
    while m <= 20 {
        if m % 4 == 0 {
            println!("m is {}", m);
        }
        m += 1;
    }

    // 반복문 for
    for i in 1..11 { // 1..11 means range(1,11)
        println!("i is {}", i);
    }

    // 이게 뭐야
    let numbers = 30..40;
    let animals = vec!["Rabbit", "Dog", "Cat"];

    for i in numbers {
        println!("i is {}", i);
    }

    for a in animals.iter() {
        print!("{} ", a)
    }

    for (index, name) in animals.iter().enumerate() {
        print!("{} : {}, ", index, name); //얽 뭐가 많아요
    }

    // enum - 열거체
    enum Direction {
        Up,
        Down,
        Left,
        Right
    }

    let player_direction:Direction = Direction::Left; // 요런식으로 정의할 수 있구나

    // switch랑 비슷한 match문 활용!
    match player_direction {
        Direction::Up => println!("Going up!"),
        Direction::Down => println!("Going down..."),
        Direction::Left => println!("tfel"),
        Direction::Right => println!("thgir"),
    }


}
