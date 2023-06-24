use std::collections::HashMap;

use uuid::Uuid;

pub trait Sessions {
    fn create_session(&mut self, user_uuid: &str) -> String;
    fn delete_session(&mut self, user_uuid: &str) -> Result<(), String>;
}

#[derive(Default)]
pub struct SessionsImpl {
    uuid_to_session: HashMap<String, String>,
}

impl Sessions for SessionsImpl {
    fn create_session(&mut self, user_uuid: &str) -> String {
        let session: String = Uuid::new_v4().to_string();

        self.uuid_to_session
            .insert(user_uuid.to_owned(), session.clone());

        session
    }

    fn delete_session(&mut self, user_uuid: &str) -> Result<(), String> {
        if self.uuid_to_session.contains_key(user_uuid) {
            self.uuid_to_session.remove(user_uuid);

            Ok(())
        } else {
            Err("session not found".to_owned())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_session() {
        let mut session_service = SessionsImpl::default();
        assert_eq!(session_service.uuid_to_session.len(), 0);
        let session = session_service.create_session("123456");
        assert_eq!(session_service.uuid_to_session.len(), 1);
        assert_eq!(
            session_service.uuid_to_session.get("123456").unwrap(),
            &session
        );
    }

    #[test]
    fn should_overwrite_existing_session() {
        let mut session_service = SessionsImpl::default();
        assert_eq!(session_service.uuid_to_session.len(), 0);
        session_service.create_session("123456");
        let session = session_service.create_session("123456");
        assert_eq!(session_service.uuid_to_session.len(), 1);
        assert_eq!(
            session_service.uuid_to_session.get("123456").unwrap(),
            &session
        );
    }

    #[test]
    fn should_delete_session() {
        let mut session_service = SessionsImpl::default();
        session_service.create_session("123456");
        session_service.delete_session("123456");
        assert_eq!(session_service.uuid_to_session.len(), 0);
    }

    #[test]
    fn deletion_should_if_session_does_not_exist() {
        let mut session_service = SessionsImpl::default();
        session_service.create_session("123456");
        let result = session_service.delete_session("invalid_session");
        assert_eq!(session_service.uuid_to_session.len(), 1);
        assert_eq!(result.is_err(), true);
    }
    
}
