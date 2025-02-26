fn main() {
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32

    // 덧셈
    let sum = 5 + 10;

    // 뺄셈
    let difference = 95.5 - 4.3;

    // 곱셈
    let product = 4 * 30;

    // 나눗셈
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // 결괏값은 -1입니다

    // 나머지 연산
    let remainder = 43 % 5;

    println!("x: {}", x);
    println!("y: {}", y);
    println!("sum: {}", sum);
    println!("difference: {}", difference);
    println!("product: {}", product);
    println!("quotient: {}", quotient);
    println!("truncated: {}", truncated);
    println!("remainder: {}", remainder);

    // 불리언
    let t = true;

    let f: bool = false; // 명시적 타입 지정

    println!("t: {}", t);
    println!("f: {}", f);

    // 문자

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    println!("c: {}", c);
    println!("z: {}", z);
    println!("heart_eyed_cat: {}", heart_eyed_cat);

    // 복합 타입
    // 튜플

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);

    let five_hundred = tup.0;

    println!("five_hundred: {}", five_hundred);

    // 배열

    let a = [1, 2, 3, 4, 5];

    let first = a[0];

    println!("The value of first is: {}", first);
}
