use async_graphql::{Context, Object, SimpleObject};
use hypersdk::hypercore::{HttpClient, PerpMarket, PriceTick, Side};



// Wrapper for GraphQL exposure
#[derive(SimpleObject)]
pub struct GqlPerpMarket {
    #[graphql(skip)]
    inner: PerpMarket,
}

#[Object]
impl GqlPerpMarket {
    async fn symbol(&self) -> &str {
        self.inner.symbol()
    }

    async fn tick_table(&self) -> &PriceTick {
        self.inner.tick_table()
    }

    async fn tick_for(&self, price: Decimal) -> Option<Decimal> {
        self.inner.tick_for(price)
    }

    async fn round_price(&self, price: Decimal) -> Option<Decimal> {
        self.inner.round_price(price)
    }

    async fn round_by_side(&self, side: Side, price: Decimal, conservative: bool) -> Option<Decimal> {
        self.inner.round_by_side(side, price, conservative)
    }

    // Add more resolvers as needed
}

pub struct Query;

#[Object]
impl Query {
    async fn perp_markets(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<GqlPerpMarket>, async_graphql::Error> {
        let client = ctx.data::<HttpClient>()?;
        let perps = client.perps().await?;
        Ok(perps.into_iter().map(|inner| GqlPerpMarket { inner }).collect())
    }
}