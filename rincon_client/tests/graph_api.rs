
extern crate tokio_core;

extern crate rincon_core;
extern crate rincon_connector;
extern crate rincon_client;
extern crate rincon_test_helper;

use rincon_test_helper::*;
use rincon_client::graph::*;

#[test]
fn create_graph() {
    arango_test_with_user_db("test_graph_user10", "test_graph_db10", |conn, ref mut core| {

        let edge_defs = vec![
            EdgeDefinition::new("UsersInGroups",
                vec!["Users".to_owned()],
                vec!["Groups".to_owned()]
            ),
        ];

        #[cfg(not(feature = "enterprise"))]
        let new_graph = NewGraph::new("test_graph1", edge_defs.clone());
        #[cfg(feature = "enterprise")]
        let new_graph = NewGraph::new("test_graph1", edge_defs.clone(), false);

        let method = CreateGraph::new(new_graph);
        let graph = core.run(conn.execute(method)).unwrap();

        assert_eq!("_graphs", graph.id().collection_name());
        assert_eq!("test_graph1", graph.id().document_key());
        assert_eq!("test_graph1", graph.key().as_str());
        assert!(!graph.revision().as_str().is_empty());
        assert_eq!("test_graph1", graph.name());
        assert_eq!(edge_defs, graph.edge_definitions());
        assert_eq!(0, graph.orphan_collections().len());

        #[cfg(feature = "enterprise")]
        assert_eq!(false, graph.is_smart());
        #[cfg(feature = "enterprise")]
        assert_eq!("", graph.smart_graph_attribute());
    });
}

#[cfg(feature = "enterprise")]
#[test]
fn create_smart_graph() {
    arango_test_with_user_db("test_graph_user11", "test_graph_db11", |conn, ref mut core| {

        let edge_defs = vec![
            EdgeDefinition::new("UsersInGroups",
                vec!["Users".to_owned()],
                vec!["Groups".to_owned()]
            ),
        ];

        let mut new_graph = NewGraph::new("test_graph1", edge_defs.clone(), true);
        new_graph.options_mut().set_smart_graph_attribute("knows".to_owned());
        let new_graph = new_graph;

        let method = CreateGraph::new(new_graph);
        let graph = core.run(conn.execute(method)).unwrap();

        assert_eq!("_graphs", graph.id().collection_name());
        assert_eq!("test_graph1", graph.id().document_key());
        assert_eq!("test_graph1", graph.key().as_str());
        assert!(!graph.revision().as_str().is_empty());
        assert_eq!("test_graph1", graph.name());
        assert_eq!(edge_defs, graph.edge_definitions());
        assert_eq!(0, graph.orphan_collections().len());

        assert_eq!(true, graph.is_smart());
        assert_eq!("knows", graph.smart_graph_attribute());
    });
}

#[test]
fn create_graph_with_2_edge_definitions() {
    arango_test_with_user_db("test_graph_user12", "test_graph_db12", |conn, ref mut core| {

        let edge_defs = vec![
            EdgeDefinition::new("UsersInGroups",
                vec!["Users".to_owned()],
                vec!["Groups".to_owned()]
            ),
            EdgeDefinition::new("UsersKnowsUsers",
                vec!["Users".to_owned()],
                vec!["Users".to_owned()]
            ),
        ];

        #[cfg(not(feature = "enterprise"))]
        let new_graph = NewGraph::new("test_graph1", edge_defs.clone());
        #[cfg(feature = "enterprise")]
        let new_graph = NewGraph::new("test_graph1", edge_defs.clone(), false);

        let method = CreateGraph::new(new_graph);
        let graph = core.run(conn.execute(method)).unwrap();

        assert_eq!("_graphs", graph.id().collection_name());
        assert_eq!("test_graph1", graph.id().document_key());
        assert_eq!("test_graph1", graph.key().as_str());
        assert!(!graph.revision().as_str().is_empty());
        assert_eq!("test_graph1", graph.name());
        assert_eq!(edge_defs, graph.edge_definitions());
        assert_eq!(0, graph.orphan_collections().len());

        #[cfg(feature = "enterprise")]
            assert_eq!(false, graph.is_smart());
        #[cfg(feature = "enterprise")]
            assert_eq!("", graph.smart_graph_attribute());
    });
}

#[test]
fn delete_graph() {
    arango_test_with_user_db("test_graph_user20", "test_graph_db20", |conn, ref mut core| {

        let edge_defs = vec![
            EdgeDefinition::new("UsersInGroups",
                vec!["Users".to_owned()],
                vec!["Groups".to_owned()]
            ),
        ];
        #[cfg(not(feature = "enterprise"))]
        let new_graph = NewGraph::new("test_graph1", edge_defs);
        #[cfg(feature = "enterprise")]
        let new_graph = NewGraph::new("test_graph1", edge_defs, false);
        let graph = core.run(conn.execute(CreateGraph::new(new_graph))).unwrap();
        assert_eq!("_graphs/test_graph1".to_owned(), graph.id().to_string());

        let method = DeleteGraph::with_name("test_graph1");
        let deleted = core.run(conn.execute(method)).unwrap();

        assert_eq!(true, deleted);
    });
}

#[test]
fn get_graph() {
    arango_test_with_user_db("test_graph_user30", "test_graph_db30", |conn, ref mut core| {

        let edge_defs = vec![
            EdgeDefinition::new("UsersInGroups",
                vec!["Users".to_owned()],
                vec!["Groups".to_owned()]
            ),
        ];
        #[cfg(not(feature = "enterprise"))]
        let new_graph = NewGraph::new("test_graph1", edge_defs);
        #[cfg(feature = "enterprise")]
        let new_graph = NewGraph::new("test_graph1", edge_defs, false);
        let created = core.run(conn.execute(CreateGraph::new(new_graph))).unwrap();
        assert_eq!("_graphs/test_graph1".to_owned(), created.id().to_string());

        let method = GetGraph::with_name("test_graph1");
        let graph = core.run(conn.execute(method)).unwrap();

        assert_eq!(created, graph);
    });
}

#[test]
fn list_graphs() {
    arango_test_with_user_db("test_graph_user40", "test_graph_db40", |conn, ref mut core| {

        let edge_defs1 = vec![
            EdgeDefinition::new("UsersInGroups",
                vec!["Users".to_owned()],
                vec!["Groups".to_owned()]
            ),
        ];
        #[cfg(not(feature = "enterprise"))]
        let new_graph1 = NewGraph::new("test_graph1", edge_defs1);
        #[cfg(feature = "enterprise")]
        let new_graph1 = NewGraph::new("test_graph1", edge_defs1, false);
        let created1 = core.run(conn.execute(CreateGraph::new(new_graph1))).unwrap();
        assert_eq!("_graphs/test_graph1".to_owned(), created1.id().to_string());

        let edge_defs2 = vec![
            EdgeDefinition::new("UsersKnowsUsers",
                vec!["Users".to_owned()],
                vec!["Users".to_owned()]
            ),
        ];
        #[cfg(not(feature = "enterprise"))]
        let new_graph2 = NewGraph::new("test_graph2", edge_defs2);
        #[cfg(feature = "enterprise")]
        let new_graph2 = NewGraph::new("test_graph2", edge_defs2, false);
        let created2 = core.run(conn.execute(CreateGraph::new(new_graph2))).unwrap();
        assert_eq!("_graphs/test_graph2".to_owned(), created2.id().to_string());

        let method = ListGraphs::new();
        let graphs = core.run(conn.execute(method)).unwrap();

        let graph1 = graphs.iter().find(|g| g.name() == "test_graph1").unwrap();
        let graph2 = graphs.iter().find(|g| g.name() == "test_graph2").unwrap();
        assert_eq!(&created1, graph1);
        assert_eq!(&created2, graph2);
    });
}
