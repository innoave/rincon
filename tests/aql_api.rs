
extern crate dotenv;
extern crate futures;
extern crate log4rs;
#[macro_use] extern crate serde_json;
extern crate tokio_core;

extern crate arangodb_client;

mod test_fixture;

use test_fixture::*;
use arangodb_client::api::method::ErrorCode;
use arangodb_client::api::query::Query;
use arangodb_client::api::types::{Empty, JsonValue};
use arangodb_client::aql::*;
use arangodb_client::collection::CreateCollection;
use arangodb_client::connection::Error;
use arangodb_client::cursor::CreateCursor;
use arangodb_client::index::{CreateIndex, HashIndex, IndexDetails, IndexIdOption, IndexKey, NewHashIndex};

#[test]
fn parse_valid_query() {
    arango_user_db_test("test_aql_user1", "test_aql_db11", |conn, ref mut core| {
        core.run(conn.execute(CreateCollection::with_name("customers"))).unwrap();
        let inserts = Query::new(
            "FOR i IN 1..21 \
              INSERT { \
                name: CONCAT('No.', i), \
                age: i + 21 \
              } IN customers"
        );
        core.run(conn.execute(CreateCursor::<Empty>::from_query(inserts))).unwrap();

        let query = String::from(
            "FOR c IN customers \
              FILTER c.age <= @age \
              LIMIT 10 \
              SORT c.name \
              RETURN c.name"
        );

        let method = ParseQuery::from_query(query);
        let parsed_query = core.run(conn.execute(method)).unwrap();

        let query_ast = ParsedQuery::new(
            vec!["customers"],
            vec!["age"],
            vec![
                ParsedAstNode::new(
                    "root",
                    None,
                    None,
                    None,
                    vec![
                        ParsedAstNode::new(
                            "for",
                            None,
                            None,
                            None,
                            vec![
                                ParsedAstNode::new(
                                    "variable",
                                    "c".to_owned(),
                                    AstNodeId(0),
                                    None,
                                    vec![]
                                ),
                                ParsedAstNode::new(
                                    "collection",
                                    "customers".to_owned(),
                                    None,
                                    None,
                                    vec![]
                                ),
                            ]
                        ),
                        ParsedAstNode::new(
                            "filter",
                            None,
                            None,
                            None,
                            vec![
                                ParsedAstNode::new(
                                    "compare <=",
                                    None,
                                    None,
                                    None,
                                    vec![
                                        ParsedAstNode::new(
                                            "attribute access",
                                            "age".to_owned(),
                                            None,
                                            None,
                                            vec![
                                                ParsedAstNode::new(
                                                    "reference",
                                                    "c".to_owned(),
                                                    AstNodeId(0),
                                                    None,
                                                    vec![]
                                                )
                                            ]
                                        ),
                                        ParsedAstNode::new(
                                            "parameter",
                                            "age".to_owned(),
                                            None,
                                            None,
                                            vec![]
                                        ),
                                    ]
                                )
                            ]
                        ),
                        ParsedAstNode::new(
                            "limit",
                            None,
                            None,
                            None,
                            vec![
                                ParsedAstNode::new(
                                    "value",
                                    None,
                                    None,
                                    JsonValue::from(0),
                                    vec![]
                                ),
                                ParsedAstNode::new(
                                    "value",
                                    None,
                                    None,
                                    JsonValue::from(10),
                                    vec![]
                                )
                            ]
                        ),
                        ParsedAstNode::new(
                            "sort",
                            None,
                            None,
                            None,
                            vec![
                                ParsedAstNode::new(
                                    "array",
                                    None,
                                    None,
                                    None,
                                    vec![
                                        ParsedAstNode::new(
                                            "sort element",
                                            None,
                                            None,
                                            None,
                                            vec![
                                                ParsedAstNode::new(
                                                    "attribute access",
                                                    "name".to_owned(),
                                                    None,
                                                    None,
                                                    vec![
                                                        ParsedAstNode::new(
                                                            "reference",
                                                            "c".to_owned(),
                                                            AstNodeId(0),
                                                            None,
                                                            vec![]
                                                        )
                                                    ]
                                                ),
                                                ParsedAstNode::new(
                                                    "value",
                                                    None,
                                                    None,
                                                    JsonValue::from(true),
                                                    vec![]
                                                ),
                                            ]
                                        )
                                    ]
                                )
                            ]
                        ),
                        ParsedAstNode::new(
                            "return",
                            None,
                            None,
                            None,
                            vec![
                                ParsedAstNode::new(
                                    "attribute access",
                                    "name".to_owned(),
                                    None,
                                    None,
                                    vec![
                                        ParsedAstNode::new(
                                            "reference",
                                            "c".to_owned(),
                                            AstNodeId(0),
                                            None,
                                            vec![]
                                        )
                                    ]
                                )
                            ]
                        )
                    ]
                )
            ]
        );

        assert_eq!(query_ast, parsed_query);

    });
}

#[test]
fn parse_invalid_query() {
    arango_user_db_test("test_aql_user2", "test_aql_db21", |conn, ref mut core| {
        core.run(conn.execute(CreateCollection::with_name("customers"))).unwrap();
        let inserts = Query::new(
            "FOR i IN 1..21 \
              INSERT { \
                name: CONCAT('No.', i), \
                age: i + 21 \
              } IN customers"
        );
        core.run(conn.execute(CreateCursor::<Empty>::from_query(inserts))).unwrap();

        let query = String::from(
            "FOR c IN customers \
              FILTER c.age = @age \
              LIMIT 2 \
              SORT c.name \
              RETURN c.name"
        );

        let method = ParseQuery::from_query(query);
        let result = core.run(conn.execute(method));

        match result {
            Err(Error::ApiError(error)) => {
                assert_eq!(400, error.status_code());
                assert_eq!(ErrorCode::QueryParse, error.error_code());
                assert_eq!("syntax error, unexpected assignment near '= @age LIMIT 2 SORT c.name RETUR...' at position 1:33", error.message());
            },
            _ => panic!("Error::ApiError expected but got {:?}", result),
        };
    });
}

#[test]
fn explain_valid_query() {
    arango_user_db_test("test_aql_user3", "test_aql_db31", |conn, ref mut core| {
        core.run(conn.execute(CreateCollection::with_name("customers"))).unwrap();
        let inserts = Query::new(
            "FOR i IN 1..21 \
              INSERT { \
                name: CONCAT('No.', i), \
                age: i + 21 \
              } IN customers"
        );
        core.run(conn.execute(CreateCursor::<Empty>::from_query(inserts))).unwrap();

        let query = Query::new(
            "FOR c IN customers \
              RETURN c"
        );

        let method = ExplainQuery::from_query(query);
        let explained_query = core.run(conn.execute(method)).unwrap();

        let explanation = ExplainedQuery::from_best_plan(
            ExecutionPlan::new(
                vec![
                    ExecutionNode::Singleton(SingletonNode::new(
                        ExecutionNodeId(1),
                        vec![],
                        1.,
                        1,
                    )),
                    ExecutionNode::EnumerateCollection(EnumerateCollectionNode::new(
                        ExecutionNodeId(2),
                        vec![ ExecutionNodeId(1) ],
                        23.,
                        21,
                        "test_aql_db31",
                        "customers",
                        ExecutionVariable::new(ExecutionVariableId(0), "c"),
                        false,
                    )),
                    ExecutionNode::Return(ReturnNode::new(
                        ExecutionNodeId(3),
                        vec![ ExecutionNodeId(2) ],
                        44.,
                        21,
                        ExecutionVariable::new(ExecutionVariableId(0), "c"),
                    )),
                ],
                Vec::<String>::new(),
                vec![
                    ExecutionCollection::new("read", "customers"),
                ],
                vec![
                    ExecutionVariable::new(ExecutionVariableId(0), "c"),
                ],
                44.,
                21,
            ),
            Vec::<String>::new(),
            ExecutionStats::new(29, 0, 1),
            true,
        );

        assert_eq!(explanation, explained_query);

    });
}

#[test]
fn explain_a_plan_with_some_optimizer_rules_applied() {
    arango_user_db_test("test_aql_user4", "test_aql_db41", |conn, ref mut core| {
        core.run(conn.execute(CreateCollection::with_name("customers"))).unwrap();
        let inserts = Query::new(
            "FOR i IN 1..21 \
              INSERT { \
                id: i, \
                name: CONCAT('No.', i), \
                age: i + 21 \
              } IN customers"
        );
        core.run(conn.execute(CreateCursor::<Empty>::from_query(inserts))).unwrap();
        let id_index = NewHashIndex::new(vec!["id".to_owned()], true, false, false);
        let index = core.run(conn.execute(CreateIndex::new("customers", id_index))).unwrap();
        let index_id = match *index.id() {
            IndexIdOption::Qualified(ref index_id) => index_id,
            _ => panic!("Qualified index id expected!"),
        };

        let query = Query::new(
            "FOR c IN customers \
              LET id = c.id \
              FILTER id == 21 \
              LET name = c.name \
              SORT c.id \
              LIMIT 1 \
              RETURN name"
        );

        let method = ExplainQuery::from_query(query);
        let explained_query = core.run(conn.execute(method)).unwrap();

        let explanation = ExplainedQuery::from_best_plan(
            ExecutionPlan::new(
                vec![
                    ExecutionNode::Singleton(SingletonNode::new(
                        ExecutionNodeId(1),
                        vec![],
                        1.,
                        1,
                    )),
                    ExecutionNode::Index(IndexNode::new(
                        ExecutionNodeId(11),
                        vec![ ExecutionNodeId(1) ],
                        1.95,
                        1,
                        "test_aql_db41",
                        "customers",
                        ExecutionVariable::new(ExecutionVariableId(0), "c"),
                        vec![
                            HashIndex::new(
                                IndexKey::new(index_id.index_key()).into(),
                                vec![ "id" ],
                                true,
                                false,
                                false,
                                1,
                            ).into(),
                        ],
                        ExecutionExpression::new(
                            "n-ary or",
                            None,
                            None,
                            None,
                            None,
                            None,
                            vec![],
                            vec![
                                ExecutionExpression::new(
                                    "n-ary and",
                                    None,
                                    None,
                                    None,
                                    None,
                                    None,
                                    vec![],
                                    vec![
                                        ExecutionExpression::new(
                                            "compare ==",
                                            None,
                                            None,
                                            None,
                                            None,
                                            None,
                                            vec![],
                                            vec![
                                                ExecutionExpression::new(
                                                    "attribute access",
                                                    "id".to_owned(),
                                                    None,
                                                    None,
                                                    None,
                                                    None,
                                                    vec![],
                                                    vec![
                                                        ExecutionExpression::new(
                                                            "reference",
                                                            "c".to_owned(),
                                                            ExecutionExpressionId(0),
                                                            None,
                                                            None,
                                                            None,
                                                            vec![],
                                                            vec![],
                                                        )
                                                    ],
                                                ),
                                                ExecutionExpression::new(
                                                    "value",
                                                    None,
                                                    None,
                                                    json!(21),
                                                    None,
                                                    None,
                                                    vec![],
                                                    vec![],
                                                ),
                                            ],
                                        ),
                                    ],
                                )
                            ],
                        ),
                        false,
                    )),
                    ExecutionNode::Calculation(CalculationNode::new(
                        ExecutionNodeId(4),
                        vec![ ExecutionNodeId(11) ],
                        2.95,
                        1,
                        ExecutionVariable::new(ExecutionVariableId(4), "3"),
                        "simple",
                        ExecutionExpression::new(
                            "compare ==",
                            None,
                            None,
                            None,
                            None,
                            None,
                            vec![],
                            vec![
                                ExecutionExpression::new(
                                    "attribute access",
                                    "id".to_owned(),
                                    None,
                                    None,
                                    None,
                                    None,
                                    vec![],
                                    vec![
                                        ExecutionExpression::new(
                                            "reference",
                                            "c".to_owned(),
                                            ExecutionExpressionId(0),
                                            None,
                                            None,
                                            None,
                                            vec![],
                                            vec![],
                                        ),
                                    ],
                                ),
                                ExecutionExpression::new(
                                    "value",
                                    None,
                                    None,
                                    json!(21),
                                    None,
                                    None,
                                    vec![],
                                    vec![],
                                ),
                            ],
                        ),
                        false,
                    )),
                    ExecutionNode::Filter(FilterNode::new(
                        ExecutionNodeId(5),
                        vec![ ExecutionNodeId(4) ],
                        3.95,
                        1,
                        ExecutionVariable::new(ExecutionVariableId(4), "3"),
                    )),
                    ExecutionNode::Limit(LimitNode::new(
                        ExecutionNodeId(9),
                        vec![ ExecutionNodeId(5) ],
                        4.95,
                        1,
                        0,
                        1,
                        false,
                    )),
                    ExecutionNode::Calculation(CalculationNode::new(
                        ExecutionNodeId(6),
                        vec![ ExecutionNodeId(9) ],
                        5.95,
                        1,
                        ExecutionVariable::new(ExecutionVariableId(2), "name"),
                        "attribute",
                        ExecutionExpression::new(
                            "attribute access",
                            "name".to_owned(),
                            None,
                            None,
                            None,
                            None,
                            vec![],
                            vec![
                                ExecutionExpression::new(
                                    "reference",
                                    "c".to_owned(),
                                    ExecutionExpressionId(0),
                                    None,
                                    None,
                                    None,
                                    vec![],
                                    vec![],
                                ),
                            ],
                        ),
                        false,
                    )),
                    ExecutionNode::Return(ReturnNode::new(
                        ExecutionNodeId(10),
                        vec![ ExecutionNodeId(6) ],
                        6.95,
                        1,
                        ExecutionVariable::new(ExecutionVariableId(2), "name"),
                    )),
                ],
                vec![
                    "move-calculations-up",
                    "remove-redundant-calculations",
                    "remove-unnecessary-calculations",
                    "move-calculations-up-2",
                    "use-indexes",
                    "use-index-for-sort",
                    "remove-unnecessary-calculations-2",
                    "move-calculations-down",
                ],
                vec![
                    ExecutionCollection::new("read", "customers"),
                ],
                vec![
                    ExecutionVariable::new(ExecutionVariableId(6), "5"),
                    ExecutionVariable::new(ExecutionVariableId(4), "3"),
                    ExecutionVariable::new(ExecutionVariableId(2), "name"),
                    ExecutionVariable::new(ExecutionVariableId(1), "id"),
                    ExecutionVariable::new(ExecutionVariableId(0), "c"),
                ],
                6.95,
                1,
            ),
            Vec::<String>::new(),
            ExecutionStats::new(29, 0, 1),
            true,
        );

        assert_eq!(explanation, explained_query);

    });
}

#[test]
fn explain_2_plans_with_some_optimizer_rules_specified() {
    arango_user_db_test("test_aql_user5", "test_aql_db51", |conn, ref mut core| {
        core.run(conn.execute(CreateCollection::with_name("customers"))).unwrap();
        let inserts = Query::new(
            "FOR i IN 1..21 \
              INSERT { \
                id: i, \
                name: CONCAT('No.', i), \
                age: i + 21 \
              } IN customers"
        );
        core.run(conn.execute(CreateCursor::<Empty>::from_query(inserts))).unwrap();
        let id_index = NewHashIndex::new(vec!["id".to_owned()], true, false, false);
        core.run(conn.execute(CreateIndex::new("customers", id_index))).unwrap();

        let query = Query::new(
            "FOR c IN customers \
              LET id = c.id \
              FILTER id == 21 \
              LET name = c.name \
              SORT c.id \
              LIMIT 1 \
              RETURN name"
        );

        let mut new_explain_query = NewExplainQuery::from(query);
        new_explain_query.options_mut().set_all_plans(true);
        new_explain_query.options_mut().set_max_number_of_plans(2);
        new_explain_query.options_mut().optimizer_mut().rules_mut()
            .exclude(OptimizerRule::All)
            .include(OptimizerRule::UseIndexForSort)
//            .include(OptimizerRule::Custom("use-index-range".to_owned()))
            ;
        let method = ExplainQuery::new(new_explain_query);
        let explained_query = core.run(conn.execute(method)).unwrap();

        let explanation = ExplainedQuery::from_multiple_plans(
            vec![
                ExecutionPlan::new(
                    vec![
                        ExecutionNode::Singleton(SingletonNode::new(
                            ExecutionNodeId(1),
                            vec![],
                            1.,
                            1,
                        )),
                        ExecutionNode::EnumerateCollection(EnumerateCollectionNode::new(
                            ExecutionNodeId(2),
                            vec![ ExecutionNodeId(1) ],
                            23.,
                            21,
                            "test_aql_db51",
                            "customers",
                            ExecutionVariable::new(ExecutionVariableId(0), "c"),
                            false,
                        )),
                        ExecutionNode::Calculation(CalculationNode::new(
                            ExecutionNodeId(3),
                            vec![ ExecutionNodeId(2) ],
                            44.,
                            21,
                            ExecutionVariable::new(ExecutionVariableId(1), "id"),
                            "attribute",
                            ExecutionExpression::new(
                                "attribute access",
                                "id".to_owned(),
                                None,
                                None,
                                None,
                                None,
                                vec![],
                                vec![
                                    ExecutionExpression::new(
                                        "reference",
                                        "c".to_owned(),
                                        ExecutionExpressionId(0),
                                        None,
                                        None,
                                        None,
                                        vec![],
                                        vec![],
                                    ),
                                ],
                            ),
                            false,
                        )),
                        ExecutionNode::Calculation(CalculationNode::new(
                            ExecutionNodeId(4),
                            vec![ ExecutionNodeId(3) ],
                            65.,
                            21,
                            ExecutionVariable::new(ExecutionVariableId(4), "3"),
                            "simple",
                            ExecutionExpression::new(
                                "compare ==",
                                None,
                                None,
                                None,
                                None,
                                None,
                                vec![],
                                vec![
                                    ExecutionExpression::new(
                                        "reference",
                                        "id".to_owned(),
                                        ExecutionExpressionId(1),
                                        None,
                                        None,
                                        None,
                                        vec![],
                                        vec![],
                                    ),
                                    ExecutionExpression::new(
                                        "value",
                                        None,
                                        None,
                                        json!(21),
                                        None,
                                        None,
                                        vec![],
                                        vec![],
                                    ),
                                ],
                            ),
                            false,
                        )),
                        ExecutionNode::Filter(FilterNode::new(
                            ExecutionNodeId(5),
                            vec![ ExecutionNodeId(4) ],
                            86.,
                            21,
                            ExecutionVariable::new(ExecutionVariableId(4), "3"),
                        )),
                        ExecutionNode::Calculation(CalculationNode::new(
                            ExecutionNodeId(6),
                            vec![ ExecutionNodeId(5) ],
                            107.,
                            21,
                            ExecutionVariable::new(ExecutionVariableId(2), "name"),
                            "attribute",
                            ExecutionExpression::new(
                                "attribute access",
                                "name".to_owned(),
                                None,
                                None,
                                None,
                                None,
                                vec![],
                                vec![
                                    ExecutionExpression::new(
                                        "reference",
                                        "c".to_owned(),
                                        ExecutionExpressionId(0),
                                        None,
                                        None,
                                        None,
                                        vec![],
                                        vec![],
                                    ),
                                ],
                            ),
                            false,
                        )),
                        ExecutionNode::Calculation(CalculationNode::new(
                            ExecutionNodeId(7),
                            vec![ ExecutionNodeId(6) ],
                            128.,
                            21,
                            ExecutionVariable::new(ExecutionVariableId(6), "5"),
                            "attribute",
                            ExecutionExpression::new(
                                "attribute access",
                                "id".to_owned(),
                                None,
                                None,
                                None,
                                None,
                                vec![],
                                vec![
                                    ExecutionExpression::new(
                                        "reference",
                                        "c".to_owned(),
                                        ExecutionExpressionId(0),
                                        None,
                                        None,
                                        None,
                                        vec![],
                                        vec![],
                                    ),
                                ],
                            ),
                            false,
                        )),
                        ExecutionNode::Sort(SortNode::new(
                            ExecutionNodeId(8),
                            vec![ ExecutionNodeId(7) ],
                            220.23866587835397,
                            21,
                        )),
                        ExecutionNode::Limit(LimitNode::new(
                            ExecutionNodeId(9),
                            vec![ ExecutionNodeId(8) ],
                            221.23866587835397,
                            1,
                            0,
                            1,
                            false,
                        )),
                        ExecutionNode::Return(ReturnNode::new(
                            ExecutionNodeId(10),
                            vec![ ExecutionNodeId(9) ],
                            222.23866587835397,
                            1,
                            ExecutionVariable::new(ExecutionVariableId(2), "name"),
                        )),
                    ],
                    Vec::<String>::new(),
//                    vec![
//                        "use-index-for-sort",
//                    ],
                    vec![
                        ExecutionCollection::new("read", "customers"),
                    ],
                    vec![
                        ExecutionVariable::new(ExecutionVariableId(6), "5"),
                        ExecutionVariable::new(ExecutionVariableId(4), "3"),
                        ExecutionVariable::new(ExecutionVariableId(2), "name"),
                        ExecutionVariable::new(ExecutionVariableId(1), "id"),
                        ExecutionVariable::new(ExecutionVariableId(0), "c"),
                    ],
                    222.23866587835397,
                    1,
                ),
            ],
            Vec::<String>::new(),
            ExecutionStats::new(1, 28, 1),
        );

        assert_eq!(explanation, explained_query);

    });
}
