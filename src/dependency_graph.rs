use std::collections::HashSet;

#[derive(Debug)]
pub struct DependencyGraph<T> {
    nodes: Vec<NodeData<T>>,
    edges: Vec<EdgeData>,
}

impl<T> DependencyGraph<T> {
    pub fn new() -> Self {
        DependencyGraph {
            nodes: Default::default(),
            edges: Default::default(),
        }
    }

    pub fn add_node(&mut self, data: T) -> NodeIndex {
        let index = self.nodes.len();

        self.nodes.push(NodeData {
            data,
            first_outgoing_edge: None,
        });

        index
    }

    pub fn get(&self, index: NodeIndex) -> Option<&T> {
        match self.nodes.get(index) {
            None => None,
            Some(node) => Some(&node.data),
        }
    }

    pub fn add_dependency(&mut self, source: NodeIndex, target: NodeIndex) {
        let index = self.edges.len();
        let node = &mut self.nodes[source];

        self.edges.push(EdgeData {
            target,
            next_outgoing_edge: node.first_outgoing_edge,
        });

        node.first_outgoing_edge = Some(index);
    }

    pub fn dependencies(&self, index: NodeIndex) -> Dependencies<T> {
        Dependencies {
            graph: self,
            current_edge_index: self.nodes[index].first_outgoing_edge,
        }
    }
}

pub type NodeIndex = usize;

#[derive(Debug)]
struct NodeData<T> {
    first_outgoing_edge: Option<EdgeIndex>,
    data: T,
}

type EdgeIndex = usize;

#[derive(Debug)]
struct EdgeData {
    target: NodeIndex,
    next_outgoing_edge: Option<EdgeIndex>,
}

pub struct Dependencies<'graph, T> {
    graph: &'graph DependencyGraph<T>,
    current_edge_index: Option<EdgeIndex>,
}

impl<'graph, T> Iterator for Dependencies<'graph, T> {
    type Item = NodeIndex;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current_edge_index {
            None => None,
            Some(index) => {
                let edge = &self.graph.edges[index];
                self.current_edge_index = edge.next_outgoing_edge;
                Some(edge.target)
            }
        }
    }
}

#[derive(Debug)]
pub enum TopologicalError {
    ContainsCycle,
}

pub fn topological<T>(graph: &DependencyGraph<T>) -> Result<Vec<NodeIndex>, TopologicalError> {
    let mut visited = HashSet::new();
    let mut visiting = HashSet::new();
    let mut ordered = Vec::new();
    let mut i = 0;

    while i < graph.nodes.len() {
        match visit(graph, i, &mut visited, &mut visiting, &mut ordered) {
            Err(err) => return Err(err),
            Ok(()) => {
                i = i + 1;
            }
        }
    }

    fn visit<T>(
        graph: &DependencyGraph<T>,
        index: NodeIndex,
        visited: &mut HashSet<NodeIndex>,
        visiting: &mut HashSet<NodeIndex>,
        ordered: &mut Vec<NodeIndex>,
    ) -> Result<(), TopologicalError> {
        if visited.contains(&index) {
            return Ok(());
        }

        if visiting.contains(&index) {
            return Err(TopologicalError::ContainsCycle);
        }

        visiting.insert(index);

        for dependency in graph.dependencies(index) {
            match visit(graph, dependency, visited, visiting, ordered) {
                Err(err) => return Err(err),
                Ok(()) => {}
            }
        }

        visiting.remove(&index);
        visited.insert(index);
        ordered.push(index);

        Ok(())
    }

    Ok(ordered)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_node_to_graph() {
        let mut graph = DependencyGraph::new();
        let data = 1;
        let id = graph.add_node(data);
        assert_eq!(&data, graph.get(id).unwrap());
    }

    #[test]
    fn add_dependency_to_node() {
        let mut graph = DependencyGraph::new();
        let a = graph.add_node("a");
        let b = graph.add_node("b");
        graph.add_dependency(a, b);
        assert_eq!(graph.dependencies(a).collect::<Vec<NodeIndex>>(), vec![b]);
    }

    #[test]
    fn topological_sort() {
        let mut graph = DependencyGraph::new();
        let a = graph.add_node("a");
        let b = graph.add_node("b");
        let c = graph.add_node("c");

        graph.add_dependency(a, b);
        graph.add_dependency(b, c);

        assert_eq!(topological(&graph).unwrap(), vec![c, b, a]);
    }
}
