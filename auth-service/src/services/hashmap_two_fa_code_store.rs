use std::collections::HashMap;

use crate::domain::{
    data_store::{LoginAttemptId, TwoFACode, TwoFACodeStore, TwoFACodeStoreError},
    email::Email,
};

#[derive(Default, Clone)]
pub struct HashmapTwoFACodeStore {
    pub codes: HashMap<Email, (LoginAttemptId, TwoFACode)>,
}

#[async_trait::async_trait]
impl TwoFACodeStore for HashmapTwoFACodeStore {
   async fn add_code(
        &mut self,
        email: Email,
                        login_attempt_id: LoginAttemptId,
        code: TwoFACode,
    ) -> Result<(), TwoFACodeStoreError> {
        let insert_result = self.codes.insert(email, (login_attempt_id, code));
        match insert_result {
            Some(_val) => Err(TwoFACodeStoreError::UnexpectedError),
            None => Ok(()),
        }
    }

    async fn remove_code(&mut self, email: &Email) -> Result<(), TwoFACodeStoreError> {
        match self.codes.remove_entry(email) {
            Some(_entry) => Ok(()),
            None => Err(TwoFACodeStoreError::UnexpectedError),
        }
    }

    async fn get_code(&self, email: &Email) -> Result<(LoginAttemptId, TwoFACode), TwoFACodeStoreError> {
        let entry = self
            .codes
            .get(email)
            .ok_or(TwoFACodeStoreError::LoginAttemptIdNotFound)?;

        Ok(entry.to_owned())
    }
}
#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{domain::{data_store::{LoginAttemptId, TwoFACode, TwoFACodeStore}, email::Email}, services::hashmap_two_fa_code_store::HashmapTwoFACodeStore};

    #[tokio::test]
    async fn tests_for_two_fa_store() {
        
        let email = Email::parse("test@email.com".to_string()).unwrap();
        let id = LoginAttemptId::default();
        let code = TwoFACode::default();
        let mut store = HashmapTwoFACodeStore{
          codes: HashMap::new(),  
        };

        // check adding the code to store works
        assert!(store.add_code(email.clone(), id.clone(), code.clone()).await.is_ok());
        // check getting the code works
        assert_eq!(store.get_code(&email).await, Ok((id, code)) );
        // check removing the code works
        assert!(store.remove_code(&email).await.is_ok());

        
    }
}
