use std::io;
// i - input o - output
fn main() {
    //    ax^2 + bx + c = 0
    // D = b^2 - 4*(a*c)
    loop {
        let mut _a_str = String::new();
        let mut _b_str = String::new();
        let mut _c_str = String::new();
        println!("введите а");
        match io::stdin().read_line(&mut _a_str) {
            Ok(_) => {}
            Err(e) => println!("ошибка ввода {}", e),
        }

        println!("введите b");
        match io::stdin().read_line(&mut _b_str) {
            Ok(_) => {}
            Err(e) => println!("ошибка ввода {}", e),
        }

        println!("введите c");
        match io::stdin().read_line(&mut _c_str) {
            Ok(_) => {}
            Err(e) => println!("ошибка ввода {}", e),
        }
        println!("{_a_str} {_b_str} {_c_str}");

        let _a: f64 = _a_str.trim().parse().unwrap();
        let _b: f64 = _b_str.trim().parse().unwrap();
        let _c: f64 = _c_str.trim().parse().unwrap();

        let _d: f64 = (_b * _b) - 4.0 * (_a * _c);

        if _d > 0.0 {
            let _x1 = ((-_b) + _d.sqrt()) / (2.0 * _a);
            let _x2 = ((-_b) - _d.sqrt()) / (2.0 * _a);
            println!(
                "Решено\n есть 2 корня\n D = {}\n корень 1 = {}\n корень 2 = {}",
                _d, _x1, _x2
            )
        }
        if _d == 0.0 {
            let _x = (-_b) / (2.0 * _a);
            println!("Решено\n есть 1 корнь\n D = 0\n корень 1 = {}", _x)
        }
        if _d < 0.0 {
            println!("корней не сущетвует!\n D = {}", _d)
        }
    }
}
// let _symbol = 'f';
// let _logic = true;
// let _nadsme = String::from("sadasd");
// let mut _age: i8 = 12;
// println!("{_nadsme}");
// if _age >= 18 {
//     println!("{_age}")
// } else {
//     println!("hehehehe]")
// }
// let mut _num = 0;

// бесконечный цыкл
// loop {
//     num += 1;
//     println!("{num}");
//     break;
// }

// while цикл
// let mut num2 = 0;

// while num2 != 21 {
//     num2 += 1;
//     println!("{}", num2)
// }

// for цикл
// for i in 0..101 {
//     println!("{}", i)
// }

// match
// let mut _num3 = 96;
// match _num3 {
//     10 => println!("10"),
//     12..=22 => {
//         println!("21");
//     }
//     _ => {
//         println!("несовпало!")
//     }
// }

// let _wnum = match _num3 {
//     2 => 1,
//     3..=132 => 10,
//     _ => 123,
// };

// let mut _name = String::new();
// match io::stdin().read_line(&mut _name) {
//     Ok(_) => {
//         println!("привет! {}", _name)
//     }
//     Err(e) => {
//         println!("ой ой ой")
//     }
// }
