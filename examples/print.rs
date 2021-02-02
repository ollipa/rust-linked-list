use linkedlist::LinkedList;

fn main() {
    let mut ll: LinkedList<u32> = LinkedList::new();
    ll.push(1);
    println!("{:?}", ll.peek());
    println!("{:?}", ll.pop());
    println!("{:?}", ll.peek());
    ll.push(1);
    ll.push(2);
    ll.push(3);
    println!("{:?}", ll.peek());
    println!("{:?}", ll.pop());
    println!("{:?}", ll.peek());
    ll.clear();
    println!("{:?}", ll);
    ll.push(4);
    ll.push(5);
    ll.push(6);
    for value in ll {
        println!("{:?}", value);
    }
}
