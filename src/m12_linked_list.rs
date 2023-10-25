
#[derive(Debug)]
struct linked_list{
head:pointer
}


#[derive(Debug)]
struct Node{
    element : i32,
    next : pointer,
}



 type pointer = Option<Box<Node>>;
#[cfg(test)]
mod test {
    use super::*;

    #[test]

    fn test_linked_list() {
        let list = Node{element:1, next : Some(Box::new(Node { element: 2, next: None}))};


     
    }
}
