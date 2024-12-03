use crate::{
    app::{
        command::create_short_url::{CreateShortUrlCommand, CreateShortUrlRepository},
        query::get_full_url::{GetFullUrlQuery, GetFullUrlRepository},
    },
    id_provider::IDProvider,
};

pub struct Container<I, R, Q>
where
    I: IDProvider,
    R: CreateShortUrlRepository,
    Q: GetFullUrlRepository,
{
    pub shorten_command: CreateShortUrlCommand<I, R>,
    pub get_full_url_query: GetFullUrlQuery<Q>,
}

impl<I, R, Q> Container<I, R, Q>
where 
    I: IDProvider,
    R: CreateShortUrlRepository,
    Q: GetFullUrlRepository,
{
    pub fn new(id_provider: I, repo: R, query: Q) -> Self {
        let shorten_command = CreateShortUrlCommand::new(id_provider, repo);
        let get_full_url_query = GetFullUrlQuery::new(query);

        Container {
            shorten_command,
            get_full_url_query,
        }
    }
}
