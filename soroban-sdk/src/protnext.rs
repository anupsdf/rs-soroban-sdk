use crate::{env::internal, unwrap::UnwrapInfallible, Env};

#[derive(Clone)]
pub struct ProtocolNext {
    env: Env,
}

impl ProtocolNext {
    #[inline(always)]
    pub(crate) fn env(&self) -> &Env {
        &self.env
    }

    #[inline(always)]
    pub(crate) fn new(env: &Env) -> ProtocolNext {
        ProtocolNext { env: env.clone() }
    }
    
    pub fn test_v21(&self) -> u64 {
        internal::Env::test_v21(self.env())
            .unwrap_infallible()
            .into()
    }
}

