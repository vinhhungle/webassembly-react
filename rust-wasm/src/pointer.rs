pub fn run() {
// primitive array
let arr1 = [1,2,3];
let arr2 = arr1;

println!("Array {:?}", (arr1, arr2));

// non primitive 
let vec1 = vec![1,2,3];
let vec2 = &vec1; // must use & to point to the resource, vec1 will no longer hold the value

println!("Vector {:?}", (&vec1, vec2));

}