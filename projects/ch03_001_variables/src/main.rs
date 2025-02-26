fn main() {
    let mut x = 5;                                      //변수는 기본적으로 불변이다. mut 키워드를 사용하여 가변성을 부여한다.
                                                        //mut를 사용하지 않는 경우 값 변경이 불가능하여 아래 코드에서 에러가 발생한다.
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;    //const 활용하여 상수 선언, 상수는 타입을 반드시 명시해야 한다.
                                                        //상수는 관례적으로 대문자와 단어 사이의 밑줄이 사용되며, 값이 변경되지 않는다.    

    let y = 5;

    let y = y + 1;  //shadowing

    {
        let y = y * 2;  //shadowing
        println!("The value of y in the inner scope is: {y}");  //블록 내의 y값
    }

    println!("The value of y is: {y}"); //블록 내의 y값이 아닌, 바깥의 y값

    let spaces = "   ";             //spaces의 선언에 mut를 사용하려 하면, 컴파일 에러가 발생한다.
    let spaces = spaces.len();  //shadowing
}