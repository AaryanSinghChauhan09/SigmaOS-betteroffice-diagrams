//! SigmaOS BetterOffice Diagrams — flowcharts, ER, UML, and sequence diagrams.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiagramKind { Flowchart, ErDiagram, Uml, SequenceDiagram, MindMap }

#[derive(Debug, Clone)]
pub struct DiagramNode { pub id: String, pub label: String }
#[derive(Debug, Clone)]
pub struct DiagramEdge { pub from: String, pub to: String, pub label: Option<String> }

#[derive(Debug)]
pub struct Diagram { pub kind: DiagramKind, pub title: String, nodes: Vec<DiagramNode>, edges: Vec<DiagramEdge> }
impl Diagram {
    pub fn new(kind: DiagramKind, title: impl Into<String>) -> Self {
        Self { kind, title: title.into(), nodes: Vec::new(), edges: Vec::new() }
    }
    pub fn add_node(&mut self, id: impl Into<String>, label: impl Into<String>) {
        self.nodes.push(DiagramNode { id: id.into(), label: label.into() });
    }
    pub fn add_edge(&mut self, from: impl Into<String>, to: impl Into<String>) {
        self.edges.push(DiagramEdge { from: from.into(), to: to.into(), label: None });
    }
    pub fn node_count(&self) -> usize { self.nodes.len() }
    pub fn edge_count(&self) -> usize { self.edges.len() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn flowchart_nodes_edges() {
        let mut d = Diagram::new(DiagramKind::Flowchart, "Boot Flow");
        d.add_node("start", "Start"); d.add_node("init", "Init"); d.add_edge("start", "init");
        assert_eq!(d.node_count(), 2); assert_eq!(d.edge_count(), 1);
    }
}
