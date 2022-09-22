use futures::Future;
use std::str::FromStr;
use web3::{
    contract::{
        tokens::{Detokenize, Tokenize},
        Contract,
    },
    types::Address,
};

pub type NewContract = web3::contract::Contract<web3::transports::Http>;

#[derive(Debug)]
pub struct CustomContract(pub NewContract);
impl<'a> CustomContract {
    #[must_use]
    pub fn new(address: String) -> Self {
        let transport_url: String =
            std::env::var("TRANSPORT_URL").expect("TRANSPORT_URL must be set");
        let http = web3::transports::Http::new(&transport_url[..]).expect("");
        let web3 = web3::Web3::new(http);
        let contract_address = Address::from_str(&address).expect("Need valid address");
        let contract =
            Contract::from_json(web3.eth(), contract_address, include_bytes!("./token.json"));

        Self(contract.expect("Need valid contract"))
    }

    pub fn query<R, P>(
        &'a self,
        command: &str,
        params: P,
    ) -> impl Future<Output = Result<R, web3::contract::Error>> + '_
    where
        R: Detokenize + 'a,
        P: Tokenize + 'a,
    {
        self.0.query(
            &command[..],
            params,
            None,
            web3::contract::Options::default(),
            None,
        )
    }
}
