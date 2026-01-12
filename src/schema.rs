use async_graphql::{
    Context, Object
};


use crate::model::PerpDex;

pub struct Query;

#[Object]
impl Query {
    async fn perp_dexs(
        &self,
        ctx: &Context,
        get: String
    ) -> PerpDex {
        
    }
}