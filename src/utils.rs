
pub fn find_element<T, F>(vec: &Vec<T>, callback: F) -> Option<T>
where
    T: Clone,
    F: Fn(&T) -> bool,
{
    for element in vec.iter() {
        if callback(element) {
            return Some(element.clone());
        }
    }
    None
}
