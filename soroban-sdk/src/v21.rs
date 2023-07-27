use crate::{env::internal, unwrap::UnwrapInfallible, BytesN, Env, TryIntoVal};

#[derive(Clone)]
pub struct V21(Env);

impl V21 {
    #[inline(always)]
    pub(crate) fn env(&self) -> &Env {
        &self.0
    }

    #[inline(always)]
    pub(crate) fn new(env: &Env) -> V21 {
        V21(env.clone())
    }
    
    pub fn test_v21(&self) -> u32 {
        internal::Env::test_v21(self.env())
            .unwrap_infallible()
            .into()
    }
}

