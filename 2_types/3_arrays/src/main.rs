fn main() {
    let my_array = [3; 5];
    assert_eq!(my_array, []);
    //^ Note that this is a compile-time error, not runtime

    let my_array = [10, 20, 30];
    assert_eq!(my_array[1], 0);
}
