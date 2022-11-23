use anchor_lang::prelude::*;
use solana_security_txt::security_txt;

#[cfg(not(feature = "no-entrypoint"))]
security_txt! {
    // Required fields
    name: "Me::Anon::0xdeep",
    project_url: "http://xyz.com",
    contacts: "
            email:security@xyz.com,
            link:https://xyz.com/security,
            discord:xyz_sec#1234
    ",
    policy: "https://xyz.com/security-policy",

    // Optional Fields
    preferred_languages: "en",
    source_code: "https://github.com/xyz/secure_note",
    encryption: "
      -----BEGIN PGP PUBLIC KEY BLOCK-----
            ........................
      -----END PGP PUBLIC KEY BLOCK-----
    ",
    auditors: "None",
    acknowledgements: "
      Hate code, not me- Anon
    "
}

declare_id!("GKP7g6D6h8d24VXUKbKQCfooFxaMjgc8ruWpMwU8RzNJ");

#[program]
pub mod secure_note {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
