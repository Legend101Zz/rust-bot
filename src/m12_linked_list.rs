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
    //    match self.head{
    //     None => self.head =Some(Box::new(Node{element , next :None})),

    //     Some(previous_head)=>{
    //         let new_head = Some(Box::new(Node{
    //             element , next: Some(previous_head)
    //         }));
    //         self.head=new_head;
    //        }

    //    } 


    let previous_head = self.head.take();
    let new_head =Box::new(Node{
        element , next : previous_head
    });

    self.head = Some(new_head)

    }

    fn remove(&mut self)-> Option<i32>{
        let previous_head = self.head.take();
        match previous_head{
            None=> None,
            Some(old_head)=>{
                self.head = old_head.next;
                Some(old_head.element)
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


        let mut list_2 = linked_list::create_empty_list();
        
        list_2.add(5);
        list_2.add(6);

        dbg!(&list_2);

        list_2.remove();

        dbg!(list_2);
    }
}
