
// identity function
// use to move a variables into scope without borrowing
pub fn id<T>(x: T) -> T {
    x
}