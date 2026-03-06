pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("EoeCWM7To3utAF8G2T8fREChRLwbc7FqjTtPfDFPtv3h");

#[allow(unused_imports)]
use solana_security_txt::security_txt;

#[cfg(not(feature = "no-entrypoint"))]
security_txt! {
    name: "Verify Prog Test",
    project_url: "https://test.test",
    contacts: "email:info@test.test, twitter:@ProgSecTest",
    policy: "https://github.com/EdelweissPirate/marchverify/blob/main/SECURITY.md",
    preferred_languages: "en",
    source_code: "https://github.com/EdelweissPirate/marchverify"
}

#[program]
pub mod marchverify {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }
}