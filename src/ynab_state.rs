use std::cell::RefCell;
use ynab_api::apis;
use ynab_api::models;

use args::*;
use types::*;

type YnabClient =
    apis::client::APIClient<::hyper_tls::HttpsConnector<hyper::client::HttpConnector>>;

struct CoreAndClient {
    core: tokio_core::reactor::Core,
    client: YnabClient,
}

//@@@ RENAME?
pub struct YnabState<'a> {
    pub matches: &'a clap::ArgMatches<'static>,
    pub global: GlobalOptions,
    core_and_client: RefCell<CoreAndClient>,
    budget_settings: RefCell<Option<models::BudgetSettings>>,
}

impl<'a> YnabState<'a> {
    pub fn new(
        super_matches: &clap::ArgMatches<'static>,
        matches: &'a clap::ArgMatches<'static>,
    ) -> Result<Self, AnyError> {
        fn ynab_client(
            core: &tokio_core::reactor::Core,
            global: &GlobalOptions,
        ) -> Result<YnabClient, AnyError> {
            let access_token = global
                .access_token
                .to_owned()
                .ok_or(Error::MissingAccessToken)?;
            let hyper = ::hyper::Client::configure()
                .connector(::hyper_tls::HttpsConnector::new(4, &core.handle()).unwrap())
                .build(&core.handle());
            let config = apis::configuration::Configuration::new(access_token, hyper);
            Ok(apis::client::APIClient::new(config))
        }

        let global = GlobalOptions::from_matches(super_matches, matches)?;
        let core = tokio_core::reactor::Core::new()?;
        let client = ynab_client(&core, &global)?;
        let core_and_client = RefCell::new(CoreAndClient { core, client });
        Ok(YnabState {
            matches,
            global,
            core_and_client,
            budget_settings: RefCell::new(None),
        })
    }

    pub fn run<T>(
        &self,
        get_work: &dyn Fn(&YnabClient) -> Box<dyn futures::Future<Item = T, Error = apis::Error>>,
    ) -> Result<T, apis::Error> {
        let work = {
            let client = &self.core_and_client.borrow().client;
            get_work(client)
        };
        self.core_and_client.borrow_mut().core.run(work)
    }

    pub fn get_budget_settings(&self) -> Result<models::BudgetSettings, AnyError> {
        // @@@ IMPLEMENT LOCAL CACHING OF SETTINGS
        // @@@ DOC: ensures settings only retrieved once
        let mut settings_opt = self.budget_settings.borrow_mut();
        if settings_opt.is_none() {
            let response = self.run(&|c| {
                c.budgets_api()
                    .get_budget_settings_by_id(&self.global.budget_id)
            })?;
            let settings = response.data().settings();
            *settings_opt = Some(settings.clone());
        }
        // @@@ FIGURE OUT HOW TO RETURN A REFERENCE
        Ok(settings_opt
            .to_owned()
            .expect("Expected settingso to be loaded"))
    }

    // pub fn get_currency_format(&self) -> Result<models::CurrencyFormat, AnyError> {
    //     self.get_budget_settings().map(|s| s.currency_format())
    // }
}
