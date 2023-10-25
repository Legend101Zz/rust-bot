#[derive(Debug)]
struct linked_list {
    head: pointer,
}

#[derive(Debug)]
struct Node {
    element: i32,
    next: pointer,
}

type pointer = Option<Box<Node>>;


impl linked_list{

    fn create_empty_list()-> linked_list{
        linked_list{head:None}
    }

    fn add(&mut self, element :i32 ){
       match self.head{
        None => self.head =Some(Box::new(Node{element , next :None})),

        Some(previous_head)=>{
            let new_head = Some(Box::new(Node{
                element , next: Some(previous_head)
            }));
            self.head=new_head;
           }

       } 

      
    }
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]

    fn test_linked_list() {
        // let list = Node{element:1, next : Some(Box::new(Node { element: 2, next: None}))};

        let list = linked_list {
            head: Some(Box::new(Node {
                element: 100,
                next: Some(Box::new(Node {
                    element: 200,
                    next: None,
                })),
            })),
        };

        dbg!(list.head.unwrap().next.unwrap());


        let list_2 = linked_list::create_empty_list();

        dbg!(list_2);
    }
}
