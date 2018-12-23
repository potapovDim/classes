fn main() {
    // присвоение
    {
        let _x = 255; // связывание имен
        let (_a, _b) = (2, 12); // шаблон
        let _a_not_mut = 5;
        // a_not_mut = 12; oшибка a_not_mut без слова mut не может быть переприсвоена
        let mut _a_mut = 19; // оператор обьявления
        _a_mut = 22;
    }
    // функции
    {
        fn _print_number(num: i32) {
            println!("{}", num);
        }
        fn _ge_sum(first_num: i32, secont_num: i32) {
            first_num + secont_num;
        }
    }
}
