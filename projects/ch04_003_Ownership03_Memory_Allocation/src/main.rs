fn main()
{
    //메모리와 할당

    //메모리는 런타임에 스택과 힙으로 분리된다

    //이동, 정수형

    let x = 5;  //5를 x에 바인딩
    let y = x;  //x 값의 복사본을 y에 바인딩
                //두 5값은 스택에 푸시된다

    println!("x = {}, y = {}", x, y);

    //이동, String
    let s1 = String::from("hello");
    let s2 = s1;    //

    //println!("{}, world!", s1); //에러 발생 : 유효하지 않은 참조자의 사용
    
    //clone 메서드
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}
