use crate::hashmap_vec::MyHashMapVec;

// mod hashmap_vec;
// mod vec_hashmap;
mod hashmap;
mod hashmap_vec;

fn main() {
    let mut my_map: MyHashMapVec<String, String> = MyHashMapVec::new();
    my_map.insert("Name".to_string(), "Ilya".to_string());
    // println!("{:?}", my_map.get("Name".to_string()));
    // println!("{:?}", my_map.get("Ilya".to_string()));
    // my_map.remove("Name".to_string());
}
