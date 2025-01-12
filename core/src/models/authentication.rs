use serde_derive::{Deserialize, Serialize};

/// Provvides the default OAUTH2 rezquest encoding
fn default_oauth2_request_encoding() -> String{
    OAuth2RequestEncoding::FORM_URL.to_string()
}

/// Provides the default OAUTH2 token endpoint
fn default_token_endpoint() -> String{
    "/oauth2/token".to_string()
}

/// Provides the default OAUTH2 revocation endpoint
fn default_revocation_endpoint() -> String{
    "/oauth2/revoke".to_string()
}

/// Provides the default OAUTH2 introspection endpoint
fn default_introspection_endpoint() -> String{
    "/oauth2/introspect".to_string()
}

/// Enumerates all supported authentication schemes
pub struct AuthenticationScheme;
impl AuthenticationScheme {
    /// Gets the Basic authentication scheme
    pub const BASIC: &'static str = "Basic";
    /// Gets the Bearer authentication scheme
    pub const BEARER: &'static str = "Bearer";
    /// Gets the Certificate authentication scheme
    pub const CERTIFICATE: &'static str = "Certificate";
    /// Gets the Digest authentication scheme
    pub const DIGEST: &'static str = "Digest";
    /// Gets the OAuth2 authentication scheme
    pub const OAUTH2: &'static str = "OAuth2";
    /// Gets the OpenIDConnect authentication scheme
    pub const OIDC: &'static str = "OpenIDConnect";
}

/// Enumerates all supported OAUTH2 authentication methods
pub struct OAuth2ClientAuthenticationMethod;
impl OAuth2ClientAuthenticationMethod{
    /// Represents the "client_secret_basic" authentication method, where the client secret is sent using HTTP Basic Authentication.
    pub const BASIC: &'static str = "client_secret_basic";
    /// Represents the "client_secret_post" authentication method, where the client secret is sent in the body of the POST request.
    pub const POST: &'static str = "client_secret_post";
    /// Represents the "client_secret_jwt" authentication method, where the client authenticates using a JWT signed with the client secret.
    pub const JWT: &'static str = "client_secret_jwt";
    /// Represents the "private_key_jwt" authentication method, where the client authenticates using a JWT signed with a private key.
    pub const PRIVATE_KEY: &'static str = "private_key_jwt";
    /// Represents the "none" authentication method, where no client authentication is performed.
    pub const NONE: &'static str = "none";
}

/// Exposes all supported request encodings for OAUTH2 requests
pub struct OAuth2RequestEncoding;
impl OAuth2RequestEncoding{
    /// Represents the "application/x-www-form-urlencoded" content type
    pub const FORM_URL: &'static str = "application/x-www-form-urlencoded";
    /// Represents the "application/json" content type
    pub const JSON: &'static str = "application/json";
}

/// Represents the definition of an authentication policy
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthenticationPolicyDefinition{

    /// Gets/sets the name of the top level authentication policy to use, if any
    #[serde(rename = "use", skip_serializing_if = "Option::is_none")]
    pub use_: Option<String>,

    /// Gets/sets the `basic` authentication scheme to use, if any
    #[serde(rename = "basic", skip_serializing_if = "Option::is_none")]
    pub basic : Option<BasicAuthenticationSchemeDefinition>,

    /// Gets/sets the `Bearer` authentication scheme to use, if any
    #[serde(rename = "bearer", skip_serializing_if = "Option::is_none")]
    pub bearer : Option<BearerAuthenticationSchemeDefinition>,

    /// Gets/sets the `Certificate` authentication scheme to use, if any
    #[serde(rename = "certificate", skip_serializing_if = "Option::is_none")]
    pub certificate : Option<CertificateAuthenticationSchemeDefinition>,

    /// Gets/sets the `Digest` authentication scheme to use, if any
    #[serde(rename = "digest", skip_serializing_if = "Option::is_none")]
    pub digest : Option<DigestAuthenticationSchemeDefinition>,

    /// Gets/sets the `OAUTH2` authentication scheme to use, if any
    #[serde(rename = "oauth2", skip_serializing_if = "Option::is_none")]
    pub oauth2 : Option<OAuth2AuthenticationSchemeDefinition>,

    /// Gets/sets the `OIDC` authentication scheme to use, if any
    #[serde(rename = "oidc", skip_serializing_if = "Option::is_none")]
    pub oidc : Option<OpenIDConnectSchemeDefinition>

}
/// A trait that all authentication schemes must implement
pub trait AuthenticationSchemeDefinition {
    /// Returns the name of the authentication scheme
    fn scheme(&self) -> &str;
}

/// Represents the definition of a basic authentication scheme
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct BasicAuthenticationSchemeDefinition{

    /// Gets/sets the name of the secret, if any, used to configure the authentication scheme
    #[serde(rename = "use", skip_serializing_if = "Option::is_none")]
    pub use_: Option<String>,

    /// Gets/sets the username used for authentication
    #[serde(rename = "username", skip_serializing_if = "Option::is_none")]
    pub username : Option<String>,

    /// Gets/sets the password used for authentication
    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    pub password : Option<String>

}
impl AuthenticationSchemeDefinition for BasicAuthenticationSchemeDefinition {
    fn scheme(&self) -> &str {
        AuthenticationScheme::BASIC
    }
}

/// Represents the definition of a bearer authentication scheme
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct BearerAuthenticationSchemeDefinition{

    /// Gets/sets the name of the secret, if any, used to configure the authentication scheme
    #[serde(rename = "use", skip_serializing_if = "Option::is_none")]
    pub use_: Option<String>,

    /// Gets/sets the bearer token used for authentication
    #[serde(rename = "token", skip_serializing_if = "Option::is_none")]
    pub token : Option<String>

}
impl AuthenticationSchemeDefinition for BearerAuthenticationSchemeDefinition{
    fn scheme(&self) -> &str {
        AuthenticationScheme::BEARER
    }
}

/// Represents the definition of a certificate authentication scheme
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct CertificateAuthenticationSchemeDefinition{

    /// Gets/sets the name of the secret, if any, used to configure the authentication scheme
    #[serde(rename = "use", skip_serializing_if = "Option::is_none")]
    pub use_: Option<String>,

}
impl AuthenticationSchemeDefinition for CertificateAuthenticationSchemeDefinition{
    fn scheme(&self) -> &str {
        AuthenticationScheme::CERTIFICATE
    }
}

/// Represents the definition of a digest authentication scheme
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct DigestAuthenticationSchemeDefinition{

    /// Gets/sets the name of the secret, if any, used to configure the authentication scheme
    #[serde(rename = "use", skip_serializing_if = "Option::is_none")]
    pub use_: Option<String>,

    /// Gets/sets the username used for authentication
    #[serde(rename = "username", skip_serializing_if = "Option::is_none")]
    pub username : Option<String>,

    /// Gets/sets the password used for authentication
    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    pub password : Option<String>

}
impl AuthenticationSchemeDefinition for DigestAuthenticationSchemeDefinition{
    fn scheme(&self) -> &str {
        AuthenticationScheme::DIGEST
    }
}

/// Represents the definition of an OAUTH2 client
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct OAuth2AuthenticationClientDefinition{

    /// Gets/sets the OAUTH2 `client_id` to use. Required if 'Authentication' has NOT been set to 'none'.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id : Option<String>,

    /// Gets/sets the OAUTH2 `client_secret` to use, if any
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret : Option<String>,

    /// Gets/sets a JWT, if any, containing a signed assertion with the application credentials
    #[serde(rename = "assertion", skip_serializing_if = "Option::is_none")]
    pub assertion : Option<String>,

    /// Gets/sets the authentication method to use to authenticate the client. Defaults to 'client_secret_post'
    #[serde(rename = "authentication", skip_serializing_if = "Option::is_none")]
    pub authentication : Option<String>,

}

/// Represents the configuration of an OAUTH2 authentication request
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct OAuth2AuthenticationRequestDefinition{

    /// Gets/sets the encoding of the authentication request. Defaults to 'application/x-www-form-urlencoded'
    #[serde(rename = "encoding", default = "default_oauth2_request_encoding")]
    pub encoding : String

}

/// Represents the definition of an OAUTH2 token
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct OAuth2TokenDefinition{

    /// Gets/sets the security token to use
    #[serde(rename = "encoding")]
    pub token : String,

    /// Gets/sets the type of security token to use
    #[serde(rename = "type")]
    pub type_ : String

}

/// Represents the configuration of OAUTH2 endpoints/// Represents the configuration of OAUTH2 endpoints
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct OAuth2AuthenticationEndpointsDefinition{

    /// Gets/sets the relative path to the token endpoint. Defaults to `/oauth2/token`
    #[serde(rename = "token", default = "default_token_endpoint")]
    pub token : String,

    /// Gets/sets the relative path to the revocation endpoint. Defaults to `/oauth2/revoke`
    #[serde(rename = "revocation", default = "default_revocation_endpoint")]
    pub revocation : String,

    /// Gets/sets the relative path to the introspection endpoint. Defaults to `/oauth2/introspect`
    #[serde(rename = "introspection", default = "default_introspection_endpoint")]
    pub introspection : String,

}

/// Represents the definition of an OAUTH2 authentication scheme
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct OAuth2AuthenticationSchemeDefinition{

    /// Gets/sets the name of the secret, if any, used to configure the authentication scheme
    #[serde(rename = "use", skip_serializing_if = "Option::is_none")]
    pub use_: Option<String>,

    /// Gets/sets the configuration of the OAUTH2 endpoints to use
    #[serde(rename = "endpoints", skip_serializing_if = "Option::is_none")]
    pub endpoints : Option<OAuth2AuthenticationEndpointsDefinition>,

    /// Gets/sets the URI that references the OAUTH2 authority to use.
    #[serde(rename = "authority", skip_serializing_if = "Option::is_none")]
    pub authority: Option<String>,

    /// Gets/sets the grant type to use.
    #[serde(rename = "grant", skip_serializing_if = "Option::is_none")]
    pub grant: Option<String>,

    /// Gets/sets the definition of the client to use.
    #[serde(rename = "client", skip_serializing_if = "Option::is_none")]
    pub client: Option<OAuth2AuthenticationClientDefinition>,

    /// Gets/sets the configuration of the authentication request to perform.
    #[serde(rename = "request", skip_serializing_if = "Option::is_none")]
    pub request: Option<OAuth2AuthenticationRequestDefinition>,

    /// Gets/sets a list of valid issuers for token checks.
    #[serde(rename = "issuers", skip_serializing_if = "Option::is_none")]
    pub issuers: Option<Vec<String>>, // Using Vec<String> for EquatableList<string>

    /// Gets/sets the scopes to request the token for.
    #[serde(rename = "scopes", skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<String>>,

    /// Gets/sets the audiences to request the token for.
    #[serde(rename = "audiences", skip_serializing_if = "Option::is_none")]
    pub audiences: Option<Vec<String>>,

    /// Gets/sets the username to use (for Password grant).
    #[serde(rename = "username", skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,

    /// Gets/sets the password to use (for Password grant).
    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,

    /// Gets/sets the token representing the identity of the party on whose behalf the request is made.
    #[serde(rename = "subject", skip_serializing_if = "Option::is_none")]
    pub subject: Option<OAuth2TokenDefinition>,

    /// Gets/sets the token representing the acting party's identity.
    #[serde(rename = "actor", skip_serializing_if = "Option::is_none")]
    pub actor: Option<OAuth2TokenDefinition>

}
impl AuthenticationSchemeDefinition for OAuth2AuthenticationSchemeDefinition{
    fn scheme(&self) -> &str {
        AuthenticationScheme::OAUTH2
    }
}

/// Represents the definition of an OpenIDConnect authentication scheme
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenIDConnectSchemeDefinition{

    /// Gets/sets the name of the secret, if any, used to configure the authentication scheme
    #[serde(rename = "use", skip_serializing_if = "Option::is_none")]
    pub use_: Option<String>,

    /// Gets/sets the URI that references the OAUTH2 authority to use.
    #[serde(rename = "authority", skip_serializing_if = "Option::is_none")]
    pub authority: Option<String>,

    /// Gets/sets the grant type to use.
    #[serde(rename = "grant", skip_serializing_if = "Option::is_none")]
    pub grant: Option<String>,

    /// Gets/sets the definition of the client to use.
    #[serde(rename = "client", skip_serializing_if = "Option::is_none")]
    pub client: Option<OAuth2AuthenticationClientDefinition>,

    /// Gets/sets the configuration of the authentication request to perform.
    #[serde(rename = "request", skip_serializing_if = "Option::is_none")]
    pub request: Option<OAuth2AuthenticationRequestDefinition>,

    /// Gets/sets a list of valid issuers for token checks.
    #[serde(rename = "issuers", skip_serializing_if = "Option::is_none")]
    pub issuers: Option<Vec<String>>, // Using Vec<String> for EquatableList<string>

    /// Gets/sets the scopes to request the token for.
    #[serde(rename = "scopes", skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<String>>,

    /// Gets/sets the audiences to request the token for.
    #[serde(rename = "audiences", skip_serializing_if = "Option::is_none")]
    pub audiences: Option<Vec<String>>,

    /// Gets/sets the username to use (for Password grant).
    #[serde(rename = "username", skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,

    /// Gets/sets the password to use (for Password grant).
    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,

    /// Gets/sets the token representing the identity of the party on whose behalf the request is made.
    #[serde(rename = "subject", skip_serializing_if = "Option::is_none")]
    pub subject: Option<OAuth2TokenDefinition>,

    /// Gets/sets the token representing the acting party's identity.
    #[serde(rename = "actor", skip_serializing_if = "Option::is_none")]
    pub actor: Option<OAuth2TokenDefinition>

}
impl AuthenticationSchemeDefinition for OpenIDConnectSchemeDefinition{
    fn scheme(&self) -> &str {
        AuthenticationScheme::OIDC
    }
}