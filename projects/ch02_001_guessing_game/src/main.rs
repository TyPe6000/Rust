use std::io;                            // io 입출력 라이브러리를 scope로 가져온다. io 라이브러리는 std 표준 라이브러리에 포함되어 있다.
use std::cmp::Ordering;                 // cmp::Ordering 열거형을 scope로 가져온다.
use rand::Rng;                          // Rng 트레이트는 난수 생성기를 구현한 메서드들을 정의한 trait                        

fn main() {                                 // main 함수는 프로그램의 시작점이다.
                                            // fn 새로운 함수를 선언 () 매개변수가 없음 {} 함수의 몸체
    println!("Guess the number!");          // println! 매크로는 텍스트를 출력하는 매크로

    let secret_number = rand::thread_rng().gen_range(1..=100); // thread_rng 함수를 호출하여 난수 생성기를 얻고 gen_range 메서드를 호출하여 1~100 사이의 난수를 생성한다.

    println!("The secret number is: {}", secret_number);       // [디버그용] 비밀번호를 출력한다.

    println!("Please input your guess.");

    loop{       // 무한 루프
        let mut guess = String::new();          // 변수 선언 let 구문, mut 가변성을 나타냄
                                            // String::new() 빈 문자열을 생성하는 함수

        io::stdin()                             // io 라이브러리의 stdin 함수를 호출
            .read_line(&mut guess)              // read_line 메서드를 호출, guess 변수를 참조로 전달
            .expect("Failed to read line");     // expect 
    
        let guess: u32 = match guess.trim().parse() {   // match 표현식, guess 변수의 값을 match 표현식으로 비교
            Ok(num) => num,                     // Ok(num) guess 변수가 숫자일 때
            Err(_) => continue,                 // Err(_) guess 변수가 숫자가 아닐 때
        };
        //let guess: u32 = guess.trim().parse().expect("Please type a number!");  // guess 변수의 타입을 u32로 Shadowing
                                                                                // trim 메서드로 문자열의 앞뒤 공백을 제거
                                                                                // parse 메서드로 문자열을 숫자로 변환
                                                                                // expect 메서드로 에러 처리

        println!("You guessed: {}", guess);     // {} placeholder, guess 변수를 출력

        match guess.cmp(&secret_number) {               // match 표현식, guess 변수와 secret_number 변수를 비교
            Ordering::Less => println!("Too small!"),   // Ordering::Less guess 변수가 작을 때 출력
            Ordering::Greater => println!("Too big!"),  // Ordering::Greater guess 변수가 클 때 출력
            Ordering::Equal => {
                println!("You win!");    // Ordering::Equal guess 변수와 secret_number 변수가 같을 때 출력
                break;                  // break 키워드로 무한 루프를 종료
            }
        }
    }
}