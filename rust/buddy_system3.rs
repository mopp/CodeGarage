/// It manages type T in 2^N (where N is order)
trait BuddyManager<T> {
    fn init(ObjectMapper<usize, *mut T>) -> BuddyManager<T> where Self: Sized;
    fn allocate(&mut self, usize) -> Option<T>;
    fn free(&mut self, T);
}

trait ObjectMapper<T, U> {
    fn to(&mut self, T) -> U;
    fn from(&mut self, U) -> T;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}
