use uuid::Uuid;

use crate::Error;

#[derive(Clone, Debug)]
pub struct Ctx {
    user_id: Uuid,
}

//  Contructor
impl Ctx {
    pub fn root_ctx() -> Self {
        Ctx {
            user_id: Uuid::try_parse("f28fb555-1cbd-4487-b80e-40c5f01c54e9")
                .ok()
                .unwrap(),
        }
    }

    pub fn new(user_id: Uuid) -> Result<Self, Error> {
        if user_id
            == Uuid::try_parse("f28fb555-1cbd-4487-b80e-40c5f01c54e9")
                .ok()
                .unwrap()
        {
            Err(Error::CtxCannotNewRootCtx)
        } else {
            Ok(Self { user_id })
        }
    }
}

// Property Accessors
impl Ctx {
    pub fn user_id(&self) -> Uuid {
        self.user_id()
    }
}
