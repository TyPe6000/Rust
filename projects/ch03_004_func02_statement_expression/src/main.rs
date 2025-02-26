fn main() {
    // 구문(statement)은 어떤 동작을 수행하고 값을 반환하지 않는 명령입니다.
    // 표현식(expression)은 결괏값을 평가합니다.

    //let y = 6; // 구문

    //let x = (let y = 6); // 구문은 표현식이 될 수 없습니다. 이 코드는 에러가 발생합니다.

    let y = 
    {
        let x = 3;
        x + 1 // 표현식. 표현식은 종결을 나타내는 semicolon(;)을 가지지 않습니다.
    };  //중괄호로 만들어진 새로운 스코프 블록도 표현식.

    println!("The value of y is: {}", y);

    let x = five(); // 함수 호출도 표현식.
    
    println!("The value of x is: {}", x);

    let x = plus_one(5); // 함수 호출도 표현식.

    println!("The value of x is: {}", x);
}

fn five() -> i32 {
    5   // 표현식
}

fn plus_one(x: i32) -> i32 {
    x + 1   // 표현식
}