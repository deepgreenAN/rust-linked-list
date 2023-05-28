fn main() {
    use linked_list::{list, List};

    let mut list = list![1, 2, 3];
    list.prepend(0);

    println!("list(debug): {:?}", list);
    println!("list length: {}", list.len());
    println!("list(display): {}", list);

    println!("pop_front: {:?}, list: {}", list.pop_front(), list);

    let vec = vec![11, 12, 13, 14];
    println!("vec: {vec:?}");

    let mut list_from_vec = vec.into_iter().collect::<List<_>>();
    println!("list from vec: {}", list_from_vec);
    println!("list[2]: {}", list_from_vec[2]);

    list_from_vec[3] = 4;
    println!("list from vec:{}", list_from_vec);

    println!("rest from 2: {}", list_from_vec[2..]);
    list_from_vec[2..] = list![300, 400, 500];
    println!("list from vec:{}", list_from_vec);
}
