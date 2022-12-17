#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RefConflict {
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub removes: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes = "vec", repeated, tag = "2")]
    pub adds: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RefTarget {
    #[prost(oneof = "ref_target::Value", tags = "1, 2")]
    pub value: ::core::option::Option<ref_target::Value>,
}
/// Nested message and enum types in `RefTarget`.
pub mod ref_target {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        #[prost(bytes, tag = "1")]
        CommitId(::prost::alloc::vec::Vec<u8>),
        #[prost(message, tag = "2")]
        Conflict(super::RefConflict),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoteBranch {
    #[prost(string, tag = "1")]
    pub remote_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub target: ::core::option::Option<RefTarget>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Branch {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Unset if the branch has been deleted locally.
    #[prost(message, optional, tag = "2")]
    pub local_target: ::core::option::Option<RefTarget>,
    /// TODO: How would we support renaming remotes while having undo work? If
    /// the remote name is stored in config, it's going to become a mess if the
    /// remote is renamed but the configs are left unchanged. Should each remote
    /// be identified (here and in configs) by a UUID?
    #[prost(message, repeated, tag = "3")]
    pub remote_branches: ::prost::alloc::vec::Vec<RemoteBranch>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GitRef {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// This field is just for historical reasons (before we had the RefTarget
    /// type). New GitRefs have (only) the target field.
    /// TODO: Delete support for the old format.
    #[prost(bytes = "vec", tag = "2")]
    pub commit_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "3")]
    pub target: ::core::option::Option<RefTarget>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tag {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub target: ::core::option::Option<RefTarget>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct View {
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub head_ids: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes = "vec", repeated, tag = "4")]
    pub public_head_ids: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[deprecated]
    #[prost(bytes = "vec", tag = "2")]
    pub wc_commit_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(map = "string, bytes", tag = "8")]
    pub wc_commit_ids: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::vec::Vec<u8>,
    >,
    #[prost(message, repeated, tag = "5")]
    pub branches: ::prost::alloc::vec::Vec<Branch>,
    #[prost(message, repeated, tag = "6")]
    pub tags: ::prost::alloc::vec::Vec<Tag>,
    /// Only a subset of the refs. For example, does not include refs/notes/.
    #[prost(message, repeated, tag = "3")]
    pub git_refs: ::prost::alloc::vec::Vec<GitRef>,
    /// This field is just for historical reasons (before we had the RefTarget
    /// type). New Views have (only) the target field.
    /// TODO: Delete support for the old format.
    #[deprecated]
    #[prost(bytes = "vec", tag = "7")]
    pub git_head_legacy: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "9")]
    pub git_head: ::core::option::Option<RefTarget>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Operation {
    #[prost(bytes = "vec", tag = "1")]
    pub view_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", repeated, tag = "2")]
    pub parents: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(message, optional, tag = "3")]
    pub metadata: ::core::option::Option<OperationMetadata>,
}
/// TODO: Share with store.proto? Do we even need the timezone here?
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Timestamp {
    #[prost(int64, tag = "1")]
    pub millis_since_epoch: i64,
    #[prost(int32, tag = "2")]
    pub tz_offset: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationMetadata {
    #[prost(message, optional, tag = "1")]
    pub start_time: ::core::option::Option<Timestamp>,
    #[prost(message, optional, tag = "2")]
    pub end_time: ::core::option::Option<Timestamp>,
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub hostname: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub username: ::prost::alloc::string::String,
    #[prost(map = "string, string", tag = "6")]
    pub tags: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
