/// It manages type T in 2^N (where N is order)
/// it is better to make T phantom type ?
/// in the case it just manages the indices.
trait BuddyManager<T> {
    fn new(ObjectMapper<usize, *mut T>) -> BuddyManager<T> where Self: Sized;
    fn allocate(&mut self, usize) -> Option<T>;
    fn free(&mut self, T);
}

/// keep target objects to manage.
trait ObjectMapper<T, U> {
    fn new(*mut U) -> ObjectMapper<T, U> where Self: Sized;
    fn to(&mut self, T) -> U;
    fn from(&mut self, U) -> T;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}
