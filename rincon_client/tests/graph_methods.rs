
#[macro_use] extern crate serde_derive;
extern crate tokio_core;

extern crate rincon_core;
extern crate rincon_connector;
extern crate rincon_client;
extern crate rincon_test_helper;

use rincon_core::api::connector::Execute;
use rincon_core::api::types::EMPTY;
use rincon_client::document::types::NewDocument;
use rincon_client::graph::methods::*;
use rincon_client::graph::types::*;

use rincon_test_helper::*;


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
fn drop_graph() {
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

        let method = DropGraph::with_name("test_graph1");
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

#[test]
fn add_vertex_collection() {
    arango_test_with_user_db("test_graph_user50", "test_graph_db50", |conn, ref mut core| {

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
        assert_eq!(0, created.orphan_collections().len());

        let vertex_collection = VertexCollection::new("other_vertices");
        let method = AddVertexCollection::new("test_graph1", vertex_collection);
        let graph = core.run(conn.execute(method)).unwrap();

        assert_eq!("_graphs", graph.id().collection_name());
        assert_eq!("test_graph1", graph.id().document_key());
        assert_eq!(1, graph.orphan_collections().len());
        assert!(graph.orphan_collections().contains(&"other_vertices".to_owned()));
    });
}

#[test]
fn remove_vertex_collection() {
    arango_test_with_user_db("test_graph_user60", "test_graph_db60", |conn, ref mut core| {

        let edge_defs = vec![
            EdgeDefinition::new("UsersInGroups",
                vec!["Users".to_owned()],
                vec!["Groups".to_owned()]
            ),
        ];
        #[cfg(not(feature = "enterprise"))]
        let new_graph = NewGraph::new("test_graph1", edge_defs)
            .with_orphan_collections(vec![
                "add_ons".to_owned(),
                "spare".to_owned()
            ]);
        #[cfg(feature = "enterprise")]
        let new_graph = NewGraph::new("test_graph1", edge_defs, false)
            .with_orphan_collections(vec![
                "add_ons".to_owned(),
                "spare".to_owned()
            ]);
        let created = core.run(conn.execute(CreateGraph::new(new_graph))).unwrap();
        assert_eq!("_graphs/test_graph1".to_owned(), created.id().to_string());
        assert_eq!(2, created.orphan_collections().len());

        let method = RemoveVertexCollection::new("test_graph1", "add_ons");
        let graph = core.run(conn.execute(method)).unwrap();

        assert_eq!("_graphs", graph.id().collection_name());
        assert_eq!("test_graph1", graph.id().document_key());
        assert_eq!(1, graph.orphan_collections().len());
        assert!(graph.orphan_collections().contains(&"spare".to_owned()));
    });
}

#[test]
fn list_vertex_collections() {
    arango_test_with_user_db("test_graph_user70", "test_graph_db70", |conn, ref mut core| {

        let edge_defs = vec![
            EdgeDefinition::new("UsersInGroups",
                vec!["Users".to_owned()],
                vec!["Groups".to_owned()]
            ),
        ];
        #[cfg(not(feature = "enterprise"))]
        let new_graph = NewGraph::new("test_graph1", edge_defs)
            .with_orphan_collections(vec![
                "add_ons".to_owned(),
                "spare".to_owned()
            ]);
        #[cfg(feature = "enterprise")]
        let new_graph = NewGraph::new("test_graph1", edge_defs, false)
            .with_orphan_collections(vec![
                "add_ons".to_owned(),
                "spare".to_owned()
            ]);
        let created = core.run(conn.execute(CreateGraph::new(new_graph))).unwrap();
        assert_eq!("_graphs/test_graph1".to_owned(), created.id().to_string());

        let method = ListVertexCollections::new("test_graph1");
        let vertices = core.run(conn.execute(method)).unwrap();

        assert!(vertices.contains(&"Users".to_owned()));
        assert!(vertices.contains(&"Groups".to_owned()));
        assert!(vertices.contains(&"add_ons".to_owned()));
        assert!(vertices.contains(&"spare".to_owned()));
        assert_eq!(4, vertices.len());
    });
}

#[test]
fn insert_vertex() {
    arango_test_with_user_db("test_graph_user120", "test_graph_db120", |conn, ref mut core| {

        let edge_defs = vec![
            EdgeDefinition::new("works_in",
                vec!["female".to_owned(), "male".to_owned()],
                vec!["city".to_owned()],
            ),
        ];
        #[cfg(not(feature = "enterprise"))]
        let new_graph = NewGraph::new("social", edge_defs);
        #[cfg(feature = "enterprise")]
        let new_graph = NewGraph::new("social", edge_defs, false);
        let created = core.run(conn.execute(CreateGraph::new(new_graph))).unwrap();
        assert_eq!("_graphs/social".to_owned(), created.id().to_string());

        #[derive(Debug, Serialize)]
        struct City {
            name: String,
        }

        let city_doc = NewDocument::from(City { name: "New Orleans".to_owned() });
        let method = InsertVertex::new("social", "city", city_doc);
        let doc_header = core.run(conn.execute(method)).unwrap();

        assert_eq!("city".to_owned(), doc_header.id().collection_name());
        assert!(!doc_header.key().as_str().is_empty());
        assert!(!doc_header.revision().as_str().is_empty());
    });
}

#[test]
fn add_edge_definition() {
    arango_test_with_user_db("test_graph_user80", "test_graph_db80", |conn, ref mut core| {

        let edge_defs = vec![
            EdgeDefinition::new("relation",
                vec!["female".to_owned(), "male".to_owned()],
                vec!["male".to_owned(), "female".to_owned()],
            ),
        ];
        #[cfg(not(feature = "enterprise"))]
        let new_graph = NewGraph::new("social", edge_defs);
        #[cfg(feature = "enterprise")]
        let new_graph = NewGraph::new("social", edge_defs, false);
        let created = core.run(conn.execute(CreateGraph::new(new_graph))).unwrap();
        assert_eq!("_graphs/social".to_owned(), created.id().to_string());

        let works_in = EdgeDefinition::new("works_in",
            vec!["female".to_owned(), "male".to_owned()],
            vec!["city".to_owned()],
        );
        let method = AddEdgeDefinition::new("social", works_in);
        let graph = core.run(conn.execute(method)).unwrap();

        assert_eq!("_graphs", graph.id().collection_name());
        assert_eq!("social", graph.id().document_key());
        assert_eq!("social", graph.name());
        let expected_edge_defs = vec![
            EdgeDefinition::new("relation",
                vec!["female".to_owned(), "male".to_owned()],
                vec!["female".to_owned(), "male".to_owned()],
            ),
            EdgeDefinition::new("works_in",
                vec!["female".to_owned(), "male".to_owned()],
                vec!["city".to_owned()],
            ),
        ];
        assert_eq!(&expected_edge_defs[..], graph.edge_definitions());
        assert!(graph.orphan_collections().is_empty());
    });
}

#[test]
fn remove_edge_definition() {
    arango_test_with_user_db("test_graph_user90", "test_graph_db90", |conn, ref mut core| {

        let edge_defs = vec![
            EdgeDefinition::new("relation",
                vec!["female".to_owned(), "male".to_owned()],
                vec!["male".to_owned(), "female".to_owned()],
            ),
            EdgeDefinition::new("works_in",
                vec!["female".to_owned(), "male".to_owned()],
                vec!["city".to_owned()],
            ),
        ];
        #[cfg(not(feature = "enterprise"))]
        let new_graph = NewGraph::new("social", edge_defs);
        #[cfg(feature = "enterprise")]
        let new_graph = NewGraph::new("social", edge_defs, false);
        let created = core.run(conn.execute(CreateGraph::new(new_graph))).unwrap();
        assert_eq!("_graphs/social".to_owned(), created.id().to_string());

        let method = RemoveEdgeDefinition::new("social", "works_in");
        let graph = core.run(conn.execute(method)).unwrap();

        assert_eq!("_graphs", graph.id().collection_name());
        assert_eq!("social", graph.id().document_key());
        assert_eq!("social", graph.name());
        let expected_edge_defs = vec![
            EdgeDefinition::new("relation",
                vec!["female".to_owned(), "male".to_owned()],
                vec!["female".to_owned(), "male".to_owned()],
            ),
        ];
        assert_eq!(&expected_edge_defs[..], graph.edge_definitions());
        assert_eq!(&vec!["city".to_owned()][..], graph.orphan_collections());
    });
}

#[test]
fn replace_edge_definition() {
    arango_test_with_user_db("test_graph_user100", "test_graph_db100", |conn, ref mut core| {

        let edge_defs = vec![
            EdgeDefinition::new("relation",
                vec!["female".to_owned(), "male".to_owned()],
                vec!["male".to_owned(), "female".to_owned()],
            ),
        ];
        #[cfg(not(feature = "enterprise"))]
        let new_graph = NewGraph::new("social", edge_defs);
        #[cfg(feature = "enterprise")]
        let new_graph = NewGraph::new("social", edge_defs, false);
        let created = core.run(conn.execute(CreateGraph::new(new_graph))).unwrap();
        assert_eq!("_graphs/social".to_owned(), created.id().to_string());

        let replacement_edge_def = EdgeDefinition::new("relation",
            vec!["female".to_owned(), "male".to_owned(), "animal".to_owned()],
            vec!["male".to_owned(), "female".to_owned(), "animal".to_owned()],
        );
        let method = ReplaceEdgeDefinition::new("social", "relation", replacement_edge_def);
        let graph = core.run(conn.execute(method)).unwrap();

        assert_eq!("_graphs", graph.id().collection_name());
        assert_eq!("social", graph.id().document_key());
        assert_eq!("social", graph.name());
        let expected_edge_defs = vec![
            EdgeDefinition::new("relation",
                vec!["animal".to_owned(), "female".to_owned(), "male".to_owned()],
                vec!["animal".to_owned(), "female".to_owned(), "male".to_owned()],
            ),
        ];
        assert_eq!(&expected_edge_defs[..], graph.edge_definitions());
        assert!(graph.orphan_collections().is_empty());
    });
}

#[test]
fn list_edge_collections() {
    arango_test_with_user_db("test_graph_user110", "test_graph_db110", |conn, ref mut core| {

        let edge_defs = vec![
            EdgeDefinition::new("relation",
                vec!["female".to_owned(), "male".to_owned()],
                vec!["male".to_owned(), "female".to_owned()],
            ),
            EdgeDefinition::new("works_in",
                vec!["female".to_owned(), "male".to_owned()],
                vec!["city".to_owned()],
            ),
        ];
        #[cfg(not(feature = "enterprise"))]
        let new_graph = NewGraph::new("social", edge_defs);
        #[cfg(feature = "enterprise")]
        let new_graph = NewGraph::new("social", edge_defs, false);
        let created = core.run(conn.execute(CreateGraph::new(new_graph))).unwrap();
        assert_eq!("_graphs/social".to_owned(), created.id().to_string());

        let method = ListEdgeCollections::new("social");
        let edges = core.run(conn.execute(method)).unwrap();

        assert!(edges.contains(&"relation".to_owned()));
        assert!(edges.contains(&"works_in".to_owned()));
        assert_eq!(2, edges.len());
    });
}

#[test]
fn insert_edge() {
    arango_test_with_user_db("test_graph_user130", "test_graph_db130", |conn, ref mut core| {

        let edge_defs = vec![
            EdgeDefinition::new("works_in",
                vec!["female".to_owned(), "male".to_owned()],
                vec!["city".to_owned()],
            ),
        ];
        #[cfg(not(feature = "enterprise"))]
        let new_graph = NewGraph::new("social", edge_defs);
        #[cfg(feature = "enterprise")]
        let new_graph = NewGraph::new("social", edge_defs, false);
        let created = core.run(conn.execute(CreateGraph::new(new_graph))).unwrap();
        assert_eq!("_graphs/social".to_owned(), created.id().to_string());

        #[derive(Debug, Serialize)]
        struct Person {
            name: String,
        }

        let person_doc = NewDocument::from(Person { name: "Jane Doe".to_owned() });
        let method = InsertVertex::new("social", "female", person_doc);
        let (person_id, _, _) = core.run(conn.execute(method)).unwrap().deconstruct();

        #[derive(Debug, Serialize)]
        struct City {
            name: String,
        }

        let city_doc = NewDocument::from(City { name: "New Orleans".to_owned() });
        let method = InsertVertex::new("social", "city", city_doc);
        let (city_id, _, _) = core.run(conn.execute(method)).unwrap().deconstruct();

        let works_in = NewEdge::new(person_id, city_id, EMPTY);
        let method = InsertEdge::new("social", "works_in", works_in);
        let edge_header = core.run(conn.execute(method)).unwrap();

        assert_eq!("works_in".to_owned(), edge_header.id().collection_name());
        assert!(!edge_header.key().as_str().is_empty());
        assert!(!edge_header.revision().as_str().is_empty());
    });
}
