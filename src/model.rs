use uuid::Uuid;

struct Agent {
    name: String,
    id: Uuid
    // TODO pubkey
}

struct Allegation {
    agent: Agent,
    timestamp: Timestamp,
    entity: Arc<Entity>,
    payload: AllegationPayload,
}

enum AllegationPayload{
    Label {
        label: String,
        context: Entity,
    },
    MemberOf {
        memberof: Arc<Entity>
    }
}

// All entities are categories
// All categories are a member of themselves?
// How do we address Hofstaedter's "pluarlization?
struct Entity {
    id: uuid
}

struct Timestamp {
    // TODO add logical timestamp
    civil_time: TS
}

// Entities should be devoid of all data payloads like language
// but we have to attach them somehow


impl Agent {
    pub fn new(name: String) -> Self {
        Agent{
            name,
            uuid: Uuid::new_v4()
        }
    }
    ///
    pub fn allege (&self, within_the_context_of: Entity, entity: Entity, is_a_member_of: Entity, circumstance: Option<Entity>){

    }
}