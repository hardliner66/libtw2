extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate uuid;

use std::collections::HashSet;

#[derive(Clone, Deserialize, Default, Eq, PartialEq, Serialize)]
pub struct Spec {
    pub constants: Vec<Constant>,
    pub game_enumerations: Vec<Enumeration>,
    pub game_flags: Vec<Flag>,
    pub game_messages: Vec<Message>,
    pub snapshot_objects: Vec<Object>,
    pub system_messages: Vec<Message>,
    pub connless_messages: Vec<ConnlessMessage>,
}

#[derive(Clone, Deserialize, Eq, PartialEq, Serialize)]
pub struct Constant {
    pub name: Identifier,
    #[serde(flatten)]
    pub value: ConstantValue,
}

#[derive(Clone, Deserialize, Eq, PartialEq, Serialize)]
pub struct Enumeration {
    pub name: Identifier,
    pub values: Vec<EnumerationValue>,
}

#[derive(Clone, Deserialize, Eq, PartialEq, Serialize)]
pub struct Flag {
    pub name: Identifier,
    pub values: Vec<FlagValue>,
}

#[derive(Clone, Deserialize, Eq, PartialEq, Serialize)]
pub struct Message {
    pub id: MessageId,
    pub name: Identifier,
    pub members: Vec<Member>,
    pub attributes: HashSet<String>,
}

#[derive(Clone, Deserialize, Eq, PartialEq, Serialize)]
pub struct Object {
    pub id: ObjectId,
    pub name: Identifier,
    pub members: Vec<Member>,
    pub attributes: HashSet<String>,
}

#[derive(Clone, Deserialize, Eq, PartialEq, Serialize)]
pub struct ConnlessMessage {
    pub id: [u8; 8],
    pub name: Identifier,
    pub members: Vec<Member>,
}

#[derive(Clone, Deserialize, Eq, PartialEq, Serialize)]
#[serde(transparent)]
pub struct Identifier {
    pub parts: Vec<String>,
}

#[derive(Clone, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "type", content = "value", rename_all = "snake_case")]
pub enum ConstantValue {
    Int32(i32),
    String(String),
}

#[derive(Clone, Deserialize, Eq, PartialEq, Serialize)]
pub struct EnumerationValue {
    pub value: i32,
    pub name: Identifier,
}

#[derive(Clone, Deserialize, Eq, PartialEq, Serialize)]
pub struct FlagValue {
    pub value: u32,
    pub name: Identifier,
}

#[derive(Clone, Deserialize, Eq, Ord, Hash, PartialEq, PartialOrd, Serialize)]
#[serde(untagged)]
pub enum MessageId {
    Numeric(i32),
    Uuid(uuid::Uuid),
}

#[derive(Clone, Deserialize, Eq, Ord, Hash, PartialEq, PartialOrd, Serialize)]
#[serde(untagged)]
pub enum ObjectId {
    Numeric(u16),
    Uuid(uuid::Uuid),
}

#[derive(Clone, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Type {
    Array(ArrayType),
    BeUint16,
    Boolean,
    Data,
    Enum(EnumType),
    Int32(Int32Type),
    Int32String,
    Optional(OptionalType),
    PackedAddresses,
    ServerinfoClient,
    Sha256,
    SnapshotObject(SnapshotObjectType),
    String(StringType),
    Tick,
    TuneParam,
    Uint8,
    Uuid,
}

#[derive(Clone, Deserialize, Eq, PartialEq, Serialize)]
pub struct ArrayType {
    pub count: i32,
    pub member_type: Box<Type>,
}

#[derive(Clone, Deserialize, Eq, PartialEq, Serialize)]
pub struct EnumType {
    #[serde(rename = "enum")]
    pub enum_: Identifier,
}

#[derive(Clone, Deserialize, Eq, PartialEq, Serialize)]
pub struct Int32Type {
    pub min: Option<i32>,
    pub max: Option<i32>,
}

#[derive(Clone, Deserialize, Eq, PartialEq, Serialize)]
pub struct OptionalType {
    pub inner: Box<Type>,
}

#[derive(Clone, Deserialize, Eq, PartialEq, Serialize)]
pub struct SnapshotObjectType {
    pub name: Identifier,
}

#[derive(Clone, Deserialize, Eq, PartialEq, Serialize)]
pub struct StringType {
    pub disallow_cc: bool,
}

#[derive(Clone, Deserialize, Eq, PartialEq, Serialize)]
pub struct Member {
    pub name: Identifier,
    #[serde(rename = "type")]
    pub type_: Type,
}
