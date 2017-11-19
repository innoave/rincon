# *ArangoDB* REST API Methods

## List of *ArangoDB* REST API Methods

Here is the list of all methods provided by the *ArangoDB* REST API.
 
The list is taken from *ArangoDB* version 3.2.7

The status column shows the status of the implementation for the low level API
in the `arangodb_client` lib. The different entries and their meanings are:

* `Ready` : method is implemented and available in the code base
* `M.x`   : method is planned to be implemented for milestone x
* an empty status means that a method is not planned to be implemented

### Administration [M.1, M.2]

| Status | Rust method struct | REST API method                     | Description |
|--------|--------------------|-------------------------------------|-------------|
| Ready  | GetTargetVersion   | GET /_admin/database/target-version | Return the required version of the database |
| M.1    |                    | GET /_admin/echo | Return current request |
|        |                    | POST /_admin/execute | Execute program |
|        |                    | GET /_admin/log | Read global logs from the server |
|        |                    | GET /_admin/log/level | Return the current server loglevel |
|        |                    | PUT /_admin/log/level | Modify and return the current server loglevel |
|        |                    | GET /_admin/long_echo | Return current request and continues |
|        |                    | POST /_admin/routing/reload | Reloads the routing information |
| M.2    |                    | GET /_admin/server/id | Return id of a server in a cluster |
| M.2    |                    | GET /_admin/server/role | Return role of a server in a cluster |
|        |                    | DELETE /_admin/shutdown | Initiate shutdown sequence |
|        |                    | GET /_admin/sleep | Sleep for a specified amount of seconds |
|        |                    | GET /_admin/statistics | Read the statistics |
|        |                    | GET /_admin/statistics-description | Statistics description |
|        |                    | POST /_admin/test | Runs tests on server |
|        |                    | GET /_admin/time | Return system time |
| M.2    |                    | GET /_api/cluster/endpoints | Get information about all coordinator endpoints |
| M.2    |                    | GET /_api/endpoint | Return list of all endpoints |
| M.2    |                    | GET /_api/engine | Return server database engine type |
|        |                    | POST /_api/tasks | creates a task |
|        |                    | GET /_api/tasks/ | Fetch all tasks or one task |
|        |                    | DELETE /_api/tasks/{id} | deletes the task with id |
|        |                    | GET /_api/tasks/{id} | Fetch one task with id |
|        |                    | PUT /_api/tasks/{id} | creates a task with id |
| Ready  | GetVersion         | GET /_api/version | Return server version |

### AQL [M.1, M.3]

| Status | Rust method struct | REST API method                     | Description |
|--------|--------------------|-------------------------------------|-------------|
| M.3    |                    | GET /_api/aqlfunction | Return registered AQL user functions
| M.3    |                    | POST /_api/aqlfunction | Create AQL user function
| M.3    |                    | DELETE /_api/aqlfunction/{name} | Remove existing AQL user function
| Ready  | ExplainQuery       | POST /_api/explain | Explain an AQL query
| Ready  | ParseQuery         | POST /_api/query | Parse an AQL query
| M.3    |                    | DELETE /_api/query-cache | Clears any results in the AQL query cache
|        |                    | GET /_api/query-cache/properties | Returns the global properties for the AQL query cache
|        |                    | PUT /_api/query-cache/properties | Globally adjusts the AQL query result cache properties
|        |                    | GET /_api/query/current | Returns the currently running AQL queries
|        |                    | GET /_api/query/properties | Returns the properties for the AQL query tracking
|        |                    | PUT /_api/query/properties | Changes the properties for the AQL query tracking
| M.3    |                    | DELETE /_api/query/slow | Clears the list of slow AQL queries
| M.3    |                    | GET /_api/query/slow | Returns the list of slow AQL queries
| M.3    |                    | DELETE /_api/query/{query-id} | Kills a running AQL query

### Bulk [M.2]

| Status | Rust method struct | REST API method                     | Description |
|--------|--------------------|-------------------------------------|-------------|
| M.2    |                    | POST /_api/batch | executes a batch request
| M.2    |                    | POST /_api/export | Create export cursor
| M.2    |                    | POST /_api/import#document | imports document values
| M.2    |                    | POST /_api/import#json | imports documents from JSON

### Cluster [not planned]

| Status | Rust method struct | REST API method                     | Description |
|--------|--------------------|-------------------------------------|-------------|
|        |                    | DELETE /_admin/cluster-test | Delete cluster roundtrip
|        |                    | GET /_admin/cluster-test | Execute cluster roundtrip
|        |                    | HEAD /_admin/cluster-test | Execute cluster roundtrip
|        |                    | PATCH /_admin/cluster-test | Update cluster roundtrip
|        |                    | POST /_admin/cluster-test | Execute cluster roundtrip
|        |                    | PUT /_admin/cluster-test | Execute cluster roundtrip
|        |                    | GET /_admin/clusterCheckPort | Check port
|        |                    | GET /_admin/clusterStatistics | Queries statistics of DBserver

### Collections [M.1]

| Status | Rust method struct         | REST API method                     | Description |
|--------|----------------------------|-------------------------------------|-------------|
| Ready  | ListCollections            | GET /_api/collection | reads all collections
| Ready  | CreateCollection           | POST /_api/collection | Create collection
| Ready  | DropCollection             | DELETE /_api/collection/{collection-name} | Drops a collection
| Ready  | GetCollection              | GET /_api/collection/{collection-name} | Return information about a collection
|        |                            | GET /_api/collection/{collection-name}/checksum | Return checksum for the collection
|        |                            | GET /_api/collection/{collection-name}/count | Return number of documents in a collection
|        |                            | GET /_api/collection/{collection-name}/figures | Return statistics for a collection
|        |                            | PUT /_api/collection/{collection-name}/load | Load collection
|        |                            | PUT /_api/collection/{collection-name}/loadIndexesIntoMemory | Load Indexes into Memory
| Ready  | GetCollectionProperties    | GET /_api/collection/{collection-name}/properties | Read properties of a collection
| Ready  | ChangeCollectionProperties | PUT /_api/collection/{collection-name}/properties | Change properties of a collection
| Ready  | RenameCollection           | PUT /_api/collection/{collection-name}/rename | Rename collection
|        |                            | GET /_api/collection/{collection-name}/revision | Return collection revision id
|        |                            | PUT /_api/collection/{collection-name}/rotate | Rotate journal of a collection
| M.1    |                            | PUT /_api/collection/{collection-name}/truncate | Truncate collection
|        |                            | PUT /_api/collection/{collection-name}/unload | Unload collection

### Cursors [M.1]

| Status | Rust method struct      | REST API method                     | Description |
|--------|-------------------------|-------------------------------------|-------------|
| Ready  | CreateCursor            | POST /_api/cursor | Create cursor
| Ready  | DeleteCursor            | DELETE /_api/cursor/{cursor-identifier} | Delete cursor
| Ready  | ReadNextBatchFromCursor | PUT /_api/cursor/{cursor-identifier} | Read next batch from cursor

### Database [M.1]

| Status | Rust method struct      | REST API method                     | Description |
|--------|-------------------------|-------------------------------------|-------------|
| Ready  | ListDatabases           | GET /_api/database | List of databases
| Ready  | CreateDatabase          | POST /_api/database | Create database
| Ready  | GetCurrentDatabase      | GET /_api/database/current | Information of the database
| Ready  | ListAccessibleDatabases | GET /_api/database/user | List of accessible databases
| Ready  | DropDatabase            | DELETE /_api/database/{database-name} | Drop database

### Documents [M.1]

| Status | Rust method struct | REST API method                     | Description |
|--------|--------------------|-------------------------------------|-------------|
| Ready  | DeleteDocument     | DELETE /_api/document/{collection} | Removes multiple documents
| Ready  | UpdateDocuments    | PATCH /_api/document/{collection} | Update documents
| Ready  | CreateDocument(s)  | POST /_api/document/{collection} | Create document
| Ready  | ReplaceDocuments   | PUT /_api/document/{collection} | Replace documents
| Ready  | DeleteDocument     | DELETE /_api/document/{document-handle} | Removes a document
| Ready  | GetDocument        | GET /_api/document/{document-handle} | Read document
| M.1    | GetDocumentHeader  | HEAD /_api/document/{document-handle} | Read document header
| Ready  | UpdateDocument     | PATCH /_api/document/{document-handle} | Update document
| Ready  | ReplaceDocument    | PUT /_api/document/{document-handle} | Replace document
|        |                    | PUT /_api/simple/all-keys | Read all documents

### Foxx [not planned]

| Status | Rust method struct | REST API method                     | Description |
|--------|--------------------|-------------------------------------|-------------|
|        |                    | GET /_api/foxx | List installed services
|        |                    | POST /_api/foxx | Install new service
|        |                    | POST /_api/foxx/commit | Commit local service state
|        |                    | GET /_api/foxx/configuration | Get configuration options
|        |                    | PATCH /_api/foxx/configuration | Update configuration options
|        |                    | PUT /_api/foxx/configuration | Replace configuration options
|        |                    | GET /_api/foxx/dependencies | Get dependency options
|        |                    | PATCH /_api/foxx/dependencies | Update dependencies options
|        |                    | PUT /_api/foxx/dependencies | Replace dependencies options
|        |                    | DELETE /_api/foxx/development | Disable development mode
|        |                    | POST /_api/foxx/development | Enable development mode
|        |                    | POST /_api/foxx/download | Download service bundle
|        |                    | GET /_api/foxx/readme | Service README
|        |                    | GET /_api/foxx/scripts | List service scripts
|        |                    | POST /_api/foxx/scripts/{name} | Run service script
|        |                    | DELETE /_api/foxx/service | Uninstall service
|        |                    | GET /_api/foxx/service | Service description
|        |                    | PATCH /_api/foxx/service | Upgrade service
|        |                    | PUT /_api/foxx/service | Replace service
|        |                    | GET /_api/foxx/swagger | Swagger description
|        |                    | POST /_api/foxx/tests | Run service tests

### Graph [M.1]

| Status | Rust method struct | REST API method                     | Description |
|--------|--------------------|-------------------------------------|-------------|
| M.1    |                    | GET /_api/gharial | List all graphs
| M.1    |                    | POST /_api/gharial | Create a graph
| M.1    |                    | DELETE /_api/gharial/{graph-name} | Drop a graph
| M.1    |                    | GET /_api/gharial/{graph-name} | Get a graph
| M.1    |                    | GET /_api/gharial/{graph-name}/edge | List edge definitions
| M.1    |                    | POST /_api/gharial/{graph-name}/edge | Add edge definition
| M.1    |                    | POST /_api/gharial/{graph-name}/edge/{collection-name} | Create an edge
| M.1    |                    | DELETE /_api/gharial/{graph-name}/edge/{collection-name}/{edge-key} | Remove an edge
| M.1    |                    | GET /_api/gharial/{graph-name}/edge/{collection-name}/{edge-key} | Get an edge
| M.1    |                    | PATCH /_api/gharial/{graph-name}/edge/{collection-name}/{edge-key} | Modify an edge
| M.1    |                    | PUT /_api/gharial/{graph-name}/edge/{collection-name}/{edge-key} | Replace an edge
| M.1    |                    | DELETE /_api/gharial/{graph-name}/edge/{definition-name} | Remove an edge definition from the graph
| M.1    |                    | PUT /_api/gharial/{graph-name}/edge/{definition-name} | Replace an edge definition
| M.1    |                    | GET /_api/gharial/{graph-name}/vertex | List vertex collections
| M.1    |                    | POST /_api/gharial/{graph-name}/vertex | Add vertex collection
| M.1    |                    | DELETE /_api/gharial/{graph-name}/vertex/{collection-name} | Remove vertex collection
| M.1    |                    | POST /_api/gharial/{graph-name}/vertex/{collection-name} | Create a vertex
| M.1    |                    | DELETE /_api/gharial/{graph-name}/vertex/{collection-name}/{vertex-key} | Remove a vertex
| M.1    |                    | GET /_api/gharial/{graph-name}/vertex/{collection-name}/{vertex-key} | Get a vertex
| M.1    |                    | PATCH /_api/gharial/{graph-name}/vertex/{collection-name}/{vertex-key} | Modify a vertex
| M.1    |                    | PUT /_api/gharial/{graph-name}/vertex/{collection-name}/{vertex-key} | Replace a vertex

### Graph edges [M.1]

| Status | Rust method struct | REST API method                     | Description |
|--------|--------------------|-------------------------------------|-------------|
| M.1    |                    | GET /_api/edges/{collection-id} | Read in- or outbound edges

### Graph Traversal [M.1]

| Status | Rust method struct | REST API method                     | Description |
|--------|--------------------|-------------------------------------|-------------|
| M.1    |                    | POST /_api/traversal | executes a traversal

### Indexes [M.1]

| Status | Rust method struct | REST API method                     | Description |
|--------|--------------------|-------------------------------------|-------------|
| Ready  | GetIndexList       | GET /_api/index | Read all indexes of a collection
| Ready  | CreateIndex        | POST /_api/index#fulltext | Create fulltext index
| Ready  | CreateIndex        | POST /_api/index#general | Create index
| Ready  | CreateIndex        | POST /_api/index#geo | Create geo-spatial index
| Ready  | CreateIndex        | POST /_api/index#hash | Create hash index
| Ready  | CreateIndex        | POST /_api/index#persistent | Create a persistent index
| Ready  | CreateIndex        | POST /_api/index#skiplist | Create skip list
| Ready  | DeleteIndex        | DELETE /_api/index/{index-handle} | Delete index
| Ready  | GetIndex           | GET /_api/index/{index-handle} | Read index

### job [not planned]

| Status | Rust method struct | REST API method                     | Description |
|--------|--------------------|-------------------------------------|-------------|
|        |                    | GET /_api/job/{job-id} | Returns async job
|        |                    | PUT /_api/job/{job-id} | Return result of an async job
|        |                    | PUT /_api/job/{job-id}/cancel | Cancel async job
|        |                    | DELETE /_api/job/{type} | Deletes async job
|        |                    | GET /_api/job/{type} | Returns list of async jobs

### Replication [not planned]

| Status | Rust method struct | REST API method                     | Description |
|--------|--------------------|-------------------------------------|-------------|
|        |                    | GET /_api/replication/applier-config | Return configuration of replication applier
|        |                    | PUT /_api/replication/applier-config | Adjust configuration of replication applier
|        |                    | PUT /_api/replication/applier-start | Start replication applier
|        |                    | GET /_api/replication/applier-state | State of the replication applier
|        |                    | PUT /_api/replication/applier-stop | Stop replication applier
|        |                    | POST /_api/replication/batch | Create new dump batch
|        |                    | DELETE /_api/replication/batch/{id} | Deletes an existing dump batch
|        |                    | PUT /_api/replication/batch/{id} | Prolong existing dump batch
|        |                    | GET /_api/replication/clusterInventory | Return cluster inventory of collections and indexes
|        |                    | GET /_api/replication/dump | Return data of a collection
|        |                    | GET /_api/replication/inventory | Return inventory of collections and indexes
|        |                    | GET /_api/replication/logger-first-tick | Returns the first available tick value
|        |                    | GET /_api/replication/logger-follow | Returns log entries
|        |                    | GET /_api/replication/logger-state | Return replication logger state
|        |                    | GET /_api/replication/logger-tick-ranges | Return the tick ranges available in the WAL logfiles
|        |                    | PUT /_api/replication/make-slave | Turn the server into a slave of another
|        |                    | GET /_api/replication/server-id | Return server id
|        |                    | PUT /_api/replication/sync | Synchronize data from a remote endpoint

### Simple Queries [not planned]

| Status | Rust method struct | REST API method                     | Description |
|--------|--------------------|-------------------------------------|-------------|
|        |                    | PUT /_api/simple/all | Return all documents
|        |                    | PUT /_api/simple/any | Return a random document
|        |                    | PUT /_api/simple/by-example | Simple query by-example
|        |                    | PUT /_api/simple/first-example | Find documents matching an example
|        |                    | PUT /_api/simple/fulltext | Fulltext index query
|        |                    | PUT /_api/simple/lookup-by-keys | Find documents by their keys
|        |                    | PUT /_api/simple/near | Returns documents near a coordinate
|        |                    | PUT /_api/simple/range | Simple range query
|        |                    | PUT /_api/simple/remove-by-example | Remove documents by example
|        |                    | PUT /_api/simple/remove-by-keys | Remove documents by their keys
|        |                    | PUT /_api/simple/replace-by-example | Replace documents by example
|        |                    | PUT /_api/simple/update-by-example | Update documents by example
|        |                    | PUT /_api/simple/within | Find documents within a radius around a coordinate
|        |                    | PUT /_api/simple/within-rectangle | Within rectangle query

### Transactions [M.2]

| Status | Rust method struct | REST API method                     | Description |
|--------|--------------------|-------------------------------------|-------------|
| M.2    |                    | POST /_api/transaction | Execute transaction

### User Management [M.1]

| Status | Rust method struct         | REST API method                     | Description |
|--------|----------------------------|-------------------------------------|-------------|
| Ready  | CreateUser                 | POST /_api/user | Create User
| Ready  | ListAvailableUsers         | GET /_api/user/ | List available Users
| Ready  | RemoveUser                 | DELETE /_api/user/{user} | Remove User
| Ready  | GetUser                    | GET /_api/user/{user} | Fetch User
| Ready  | ModifyUser                 | PATCH /_api/user/{user} | Modify User
| Ready  | ReplaceUser                | PUT /_api/user/{user} | Replace User
| Ready  | ListDatabasesForUser       | GET /_api/user/{user}/database/ | List the accessible databases for a user
| Ready  | GetDatabaseAccessLevel     | GET /_api/user/{user}/database/{database} | Get the database access level
| Ready  | GetCollectionAccessLevel   | GET /_api/user/{user}/database/{database}/{collection} | Get the specific collection access level
| Ready  | ResetDatabaseAccessLevel   | DELETE /_api/user/{user}/database/{dbname} | Clear the database access level
| Ready  | SetDatabaseAccessLevel     | PUT /_api/user/{user}/database/{dbname} | Set the database access level
| Ready  | ResetCollectionAccessLevel | DELETE /_api/user/{user}/database/{dbname}/{collection} | Clear the collection access level
| Ready  | SetCollectionAccessLevel   | PUT /_api/user/{user}/database/{dbname}/{collection} | Set the collection access level

### wal [not planned]

| Status | Rust method struct | REST API method                     | Description |
|--------|--------------------|-------------------------------------|-------------|
|        |                    | PUT /_admin/wal/flush | Flushes the write-ahead log
|        |                    | GET /_admin/wal/properties | Retrieves the configuration of the write-ahead log
|        |                    | PUT /_admin/wal/properties | Configures the write-ahead log
|        |                    | GET /_admin/wal/transactions | Returns information about the currently running transactions

