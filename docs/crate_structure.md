# Rincon Crate Structure

@startuml
artifact rincon {
    artifact rincon_core
    artifact rincon_aql
    artifact rincon_client
    artifact rincon_connector
    artifact rincon_session
    rincon_aql --> rincon_core
    rincon_client --> rincon_core
    rincon_connector --> rincon_core
    rincon_session --> rincon_aql
    rincon_session --> rincon_client
    rincon_session --> rincon_connector
}
@enduml
