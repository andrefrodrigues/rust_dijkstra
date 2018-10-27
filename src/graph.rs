//defining as graph[n] an edge to graph[n].0 node with cost graph[n].1
pub mod graph{
    pub struct Graph {
        graph: Vec<Vec<(i32,i32)>>
    }
    impl Graph{
        pub fn new() -> Graph {
            Graph { graph: Vec::new() }
        }
        pub fn add_node(&mut self, origin:i32,destination:i32,cost:i32){
            self.graph
        }
    }
}