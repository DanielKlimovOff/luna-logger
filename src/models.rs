use chrono::{DateTime, Utc};

pub struct UserMarker;
pub struct WorkspaceMarker;
pub struct ListMarker;
pub struct TaskMarker;
pub struct SubjectMarker;

pub struct Id<M> (
    i64,
    std::marker::PhantomData<M>,
);

pub type UserId = Id<UserMarker>;
pub type WorkspaceId = Id<WorkspaceMarker>;
pub type ListId = Id<ListMarker>;
pub type TaskId = Id<TaskMarker>;
pub type SubjectId = Id<SubjectMarker>;

pub enum ActionCategory {
    User,
    Workspace,
    List,
    Task,
}

pub struct ActionName (String);

pub struct Action {
    name: ActionName,
    category: ActionCategory,
}

pub struct Log {
    date: DateTime<Utc>,
    user_id: UserId,
    subject_id: SubjectId,
    action: Action,
}