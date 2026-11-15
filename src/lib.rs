#[derive(Debug, Clone, PartialEq)]
pub struct DiagramNode {
    pub id: String,
    pub label: String,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DiagramEdge {
    pub from_id: String,
    pub to_id: String,
    pub edge_type: String,
}

#[derive(Debug, Default)]
pub struct DiagramGraph {
    pub nodes: Vec<DiagramNode>,
    pub edges: Vec<DiagramEdge>,
}

impl DiagramGraph {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_node(&mut self, id: impl Into<String>, label: impl Into<String>, x: f64, y: f64) {
        self.nodes.push(DiagramNode {
            id: id.into(),
            label: label.into(),
            x,
            y,
            width: 120.0,
            height: 60.0,
        });
    }

    pub fn add_edge(
        &mut self,
        from: impl Into<String>,
        to: impl Into<String>,
        edge_type: impl Into<String>,
    ) {
        self.edges.push(DiagramEdge {
            from_id: from.into(),
            to_id: to.into(),
            edge_type: edge_type.into(),
        });
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diagram_graph() {
        // Arrange
        let mut graph = DiagramGraph::new();
        // Act
        graph.add_node("n1", "Start", 0.0, 0.0);
        graph.add_node("n2", "Process", 150.0, 0.0);
        graph.add_edge("n1", "n2", "flow");
        // Assert
        assert_eq!(graph.node_count(), 2);
        assert_eq!(graph.edges.len(), 1);
    }
}

/// Alternate diagram domain model preserved from the legacy scaffold branch.
pub mod diagram_model;
