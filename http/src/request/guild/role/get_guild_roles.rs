use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::{marker::ListBody, ResponseFuture},
    routing::Route,
};
use twilight_model::{
    guild::Role,
    id::{marker::GuildMarker, Id},
};

/// Get the roles of a guild.
#[must_use = "requests must be configured and executed"]
pub struct GetGuildRoles<'a> {
    guild_id: Id<GuildMarker>,
    http: &'a Client,
}

impl<'a> GetGuildRoles<'a> {
    pub(crate) const fn new(http: &'a Client, guild_id: Id<GuildMarker>) -> Self {
        Self { guild_id, http }
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<ListBody<Role>> {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for GetGuildRoles<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route(&Route::GetGuildRoles {
            guild_id: self.guild_id.get(),
        }))
    }
}
