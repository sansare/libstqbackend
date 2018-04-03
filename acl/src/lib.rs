//! This crate provides common ACL facilities, namely the common groups and traits.

extern crate diesel;
extern crate r2d2;
extern crate r2d2_diesel;

use diesel::pg::PgConnection;
use r2d2_diesel::ConnectionManager;

pub type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

/// Implement this trait on resource to signal if it's in the current scope
pub trait CheckScope<Scope, T> {
    fn is_in_scope(&self, user_id: i32, scope: &Scope, obj: Option<&T>) -> bool;
}

/// Access control layer for repos. It tells if a user can do a certain action with
/// certain resource. All logic for roles and permissions should be hardcoded into implementation
/// of this trait.
pub trait Acl<Resource, Action, Scope, Error, T> {
    /// Tells if a user with id `user_id` can do `action` on `resource`.
    /// `resource_with_scope` can tell if this resource is in some scope, which is also a part of `acl` for some
    /// permissions. E.g. You can say that a user can do `Create` (`Action`) on `Store` (`Resource`) only if he's the
    /// `Owner` (`Scope`) of the store.
    fn allows(
        &self,
        resource: &Resource,
        action: &Action,
        scope_checker: &CheckScope<Scope,T>,
        obj: Option<&T>,
    ) -> Result<bool, Error>;
}

/// `SystemACL` allows all manipulation with resources in all cases.
#[derive(Clone, Debug, Default)]
pub struct SystemACL {}

#[allow(unused)]
impl<Resource, Action, Scope, Error, T> Acl<Resource, Action, Scope, Error, T> for SystemACL {
    fn allows(
        &self,
        resource: &Resource,
        action: &Action,
        scope_checker: &CheckScope<Scope,T>,
        obj: Option<&T>,
    ) -> Result<bool, Error> {
        Ok(true)
    }
}

/// `UnauthorizedACL` denies all manipulation with resources in all cases.
#[derive(Clone, Debug, Default)]
pub struct UnauthorizedACL {}

#[allow(unused)]
impl<Resource, Action, Scope, Error, T> Acl<Resource, Action, Scope, Error, T> for UnauthorizedACL {
    fn allows(
        &self,
        resource: &Resource,
        action: &Action,
        scope_checker: &CheckScope<Scope,T>,
        obj: Option<&T>,
    ) -> Result<bool, Error> {
        Ok(false)
    }
}

pub trait RolesCache : Clone + Send + 'static {
    type Role;

    fn get(&self, user_id: i32) -> Vec<Self::Role>;
    fn clear(&self);
    fn remove(&self, user_id: i32);
    fn contains(&self, user_id: i32) -> bool;
    fn add_roles(&self, user_id: i32, roles: &Vec<Self::Role>);
}