use std::slice;

/// This function divides a list of student roll number in two parts.
fn main() {

    let mut student_list = vec![1, 2, 3, 4, 5, 6];

    let borrowed_list = &mut student_list[..];

    let (half_list1, half_list2) = split_at_mut(borrowed_list,3);

      println!("{:?} || {:?}",half_list1,half_list2);

}
/// Split the vector from a given index.
///
/// @params List of roll numbers.
///
/// @params Index to divide list of roll numbers in two parts.
///
/// @return This function returns tuple containing lists of roll number.
fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (slice::from_raw_parts_mut(ptr, mid),
         slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid))
    }
}

#[cfg(test)]
mod tests {
    use crate::split_at_mut;

    #[test]
    fn it_works() {
        let mut simple_list = vec![1, 2, 3, 4, 5, 6];
        let borrowed_list = &mut simple_list[..];
        assert_eq!(&mut [1, 2, 3],split_at_mut(borrowed_list,3).0);
    }

    #[test]
    #[should_panic]
    fn it_fails() {
        let mut simple_list = vec![1, 2, 3, 4, 5, 6];
        let borrowed_list = &mut simple_list[..];
        split_at_mut(borrowed_list,7);
    }
}