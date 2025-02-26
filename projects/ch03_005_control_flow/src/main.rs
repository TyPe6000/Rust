fn main() {
    // if, else if, else 조건문
    let number = 6;
    /*
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    */
    
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // let 구문에서 if 사용하기기
    let condition = true;
    // if는 표현식이기 때문에 let 구문의 우변에 사용할 수 있다.
    let number = if condition 
    {
        5
    }
    else 
    {
        6
    };

    println!("The value of number is: {}", number);

    // 반복문
    // loop 키워드를 사용하여 무한 루프를 만들 수 있다.
    // break 키워드를 사용하여 루프를 빠져나올 수 있다.
    /*
    loop 
    {
        println!("again!");
    }
    */

    //반복문에서 값 반환
    let mut counter = 0;

    let result = loop 
    {
        counter += 1;

        if counter == 10 
        {
            break counter * 2;  // break로 루프를 빠져나오면서 값을 반환
        }
    };

    println!("The result is {}", result);

    // loop label 사용하기
    // 루프 라벨로 여러 반복문 사이에 모호함 없애기
    let mut count = 0;

    'counting_up: loop 
    {
        println!("count = {}", count);
        let mut remaining = 10;

        loop 
        {
            println!("remaining = {}", remaining);
            if remaining == 9 
            {
                break;
            }

            if count == 2 
            {
                break 'counting_up;
            }

            remaining -= 1;
        }

        count += 1;
    }

    println!("End of count = {}", count);

    // while을 이용한 조건 반복문
    let mut number = 3;

    while number != 0
    {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");

    // for를 이용한 반복문
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 
    {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    for element in a {
        println!("the value is: {}", element);
    }

    // 범위를 이용한 반복문
    for number in (1..4).rev() 
    {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
