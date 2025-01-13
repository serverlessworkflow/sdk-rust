use serverless_workflow_core::models::authentication::*;

/// Represents a service used to build AuthenticationPolicyDefinitions
pub struct AuthenticationPolicyDefinitionBuilder{
    reference: Option<String>,
    builder: Option<AuthenticationSchemeBuilder>
}
impl AuthenticationPolicyDefinitionBuilder {
    
    /// Initializes a new AuthenticationPolicyDefinition
    pub fn new() -> Self{
        Self { 
            reference: None,
            builder: None 
        }
    }

    /// Sets the name of the top-level authentication policy to use
    pub fn use_(mut self, reference: &str){
        self.reference = Some(reference.to_string());
    }

    /// Configures the policy to use 'Basic' authentication
    pub fn basic(&mut self) -> &mut BasicAuthenticationSchemeDefinitionBuilder{
        let builder = BasicAuthenticationSchemeDefinitionBuilder::new();
        self.builder = Some(AuthenticationSchemeBuilder::Basic(builder));
        if let Some(AuthenticationSchemeBuilder::Basic(ref mut builder)) = self.builder{
            builder
        }
        else {
            unreachable!("Builder should always be set to Basic");
        }
    }

    /// Configures the policy to use 'Bearer' authentication
    pub fn bearer(&mut self) -> &mut BearerAuthenticationSchemeDefinitionBuilder{
        let builder = BearerAuthenticationSchemeDefinitionBuilder::new();
        self.builder = Some(AuthenticationSchemeBuilder::Bearer(builder));
        if let Some(AuthenticationSchemeBuilder::Bearer(ref mut builder)) = self.builder{
            builder
        }
        else {
            unreachable!("Builder should always be set to Bearer");
        }
    }

    /// Configures the policy to use 'Certificate' authentication
    pub fn certificate(&mut self) -> &mut CertificateAuthenticationSchemeDefinitionBuilder{
        let builder = CertificateAuthenticationSchemeDefinitionBuilder::new();
        self.builder = Some(AuthenticationSchemeBuilder::Certificate(builder));
        if let Some(AuthenticationSchemeBuilder::Certificate(ref mut builder)) = self.builder{
            builder
        }
        else {
            unreachable!("Builder should always be set to Certificate");
        }
    }

    /// Configures the policy to use 'Digest' authentication
    pub fn digest(&mut self) -> &mut DigestAuthenticationSchemeDefinitionBuilder{
        let builder = DigestAuthenticationSchemeDefinitionBuilder::new();
        self.builder = Some(AuthenticationSchemeBuilder::Digest(builder));
        if let Some(AuthenticationSchemeBuilder::Digest(ref mut builder)) = self.builder{
            builder
        }
        else {
            unreachable!("Builder should always be set to Digest");
        }
    }

    /// Configures the policy to use 'OAUTH2' authentication
    pub fn oauth2(&mut self) -> &mut OAuth2AuthenticationSchemeDefinitionBuilder{
        let builder = OAuth2AuthenticationSchemeDefinitionBuilder::new();
        self.builder = Some(AuthenticationSchemeBuilder::OAUTH2(builder));
        if let Some(AuthenticationSchemeBuilder::OAUTH2(ref mut builder)) = self.builder{
            builder
        }
        else {
            unreachable!("Builder should always be set to OAUTH2");
        }
    }

    /// Configures the policy to use 'OpenIdConnect' authentication
    pub fn oidc(&mut self) -> &mut OpenIDConnectSchemeDefinitionBuilder{
        let builder = OpenIDConnectSchemeDefinitionBuilder::new();
        self.builder = Some(AuthenticationSchemeBuilder::OIDC(builder));
        if let Some(AuthenticationSchemeBuilder::OIDC(ref mut builder)) = self.builder{
            builder
        }
        else {
            unreachable!("Builder should always be set to OpenIdConnect");
        }
    }

    /// Builds the configured AuthenticationPolicyDefinition
    pub fn build(self) -> AuthenticationPolicyDefinition{
        if self.reference.is_some(){
            let mut authentication = AuthenticationPolicyDefinition::default();
            authentication.use_ = self.reference;
            authentication
        }
        else{
            if let Some(builder) = self.builder {
                match builder {
                    AuthenticationSchemeBuilder::Basic(builder) => builder.build(),
                    AuthenticationSchemeBuilder::Bearer(builder) => builder.build(),
                    AuthenticationSchemeBuilder::Certificate(builder) => builder.build(),
                    AuthenticationSchemeBuilder::Digest(builder) => builder.build(),
                    AuthenticationSchemeBuilder::OAUTH2(builder) => builder.build(),
                    AuthenticationSchemeBuilder::OIDC(builder) => builder.build()
                } 
            } 
            else {
                panic!("The authentication policy must be configured");
            }
        }
    }

}

/// Enumerates all supported authentication scheme builders
pub enum AuthenticationSchemeBuilder{
    Basic(BasicAuthenticationSchemeDefinitionBuilder),
    Bearer(BearerAuthenticationSchemeDefinitionBuilder),
    Certificate(CertificateAuthenticationSchemeDefinitionBuilder),
    Digest(DigestAuthenticationSchemeDefinitionBuilder),
    OAUTH2(OAuth2AuthenticationSchemeDefinitionBuilder),
    OIDC(OpenIDConnectSchemeDefinitionBuilder)
}

/// Represents the service used to build BasicAuthenticationSchemeDefinitions
pub struct BasicAuthenticationSchemeDefinitionBuilder{
    scheme: BasicAuthenticationSchemeDefinition
}
impl BasicAuthenticationSchemeDefinitionBuilder{

    /// Initializes a new BasicAuthenticationSchemeDefinitionBuilder
    pub fn new() -> Self{
        Self { scheme: BasicAuthenticationSchemeDefinition::default() }
    }

    /// Configures the authentication scheme to load from the specified secret
    pub fn use_secret(mut self, secret: &str){
        self.scheme.use_ = Some(secret.to_string());
    }

    /// Sets the username to use
    pub fn with_username(&mut self, username: &str) -> &mut Self{
        self.scheme.username = Some(username.to_string());
        self
    }

    /// Sets the password to use
    pub fn with_password(&mut self, password: &str) -> &mut Self{
        self.scheme.password = Some(password.to_string());
        self
    }

    /// Builds the configured AuthenticationPolicyDefinition 
    pub fn build(self) -> AuthenticationPolicyDefinition{
        let mut authentication = AuthenticationPolicyDefinition::default();
        authentication.basic = Some(self.scheme);
        authentication
    }

}

/// Represents the service used to build BearerAuthenticationSchemeDefinitions
pub struct BearerAuthenticationSchemeDefinitionBuilder{
    scheme: BearerAuthenticationSchemeDefinition
}
impl BearerAuthenticationSchemeDefinitionBuilder{

    /// Initializes a new BearerAuthenticationSchemeDefinitionBuilder
    pub fn new() -> Self{
        Self { scheme: BearerAuthenticationSchemeDefinition::default() }
    }

    /// Configures the authentication scheme to load from the specified secret
    pub fn use_secret(&mut self, secret: &str){
        self.scheme.use_ = Some(secret.to_string());
    }

    /// Sets the bearer token to use
    pub fn with_token(&mut self, token: &str) -> &mut Self{
        self.scheme.token = Some(token.to_string());
        self
    }

    /// Builds the configured AuthenticationPolicyDefinition 
    pub fn build(self) -> AuthenticationPolicyDefinition{
        let mut authentication = AuthenticationPolicyDefinition::default();
        authentication.bearer = Some(self.scheme);
        authentication
    }

}

/// Represents the service used to build CertificateAuthenticationSchemeDefinitions
pub struct  CertificateAuthenticationSchemeDefinitionBuilder{
    scheme: CertificateAuthenticationSchemeDefinition
}
impl CertificateAuthenticationSchemeDefinitionBuilder{

    /// Initializes a new CertificateAuthenticationSchemeDefinitionBuilder
    pub fn new() -> Self{
        Self { scheme: CertificateAuthenticationSchemeDefinition::default() }
    }

    /// Configures the authentication scheme to load from the specified secret
    pub fn use_secret(mut self, secret: &str){
        self.scheme.use_ = Some(secret.to_string());
    }

    /// Builds the configured AuthenticationPolicyDefinition 
    pub fn build(self) -> AuthenticationPolicyDefinition{
        let mut authentication = AuthenticationPolicyDefinition::default();
        authentication.certificate = Some(self.scheme);
        authentication
    }

}

/// Represents the service used to build DigestAuthenticationSchemeDefinitions
pub struct DigestAuthenticationSchemeDefinitionBuilder{
    scheme: DigestAuthenticationSchemeDefinition
}
impl DigestAuthenticationSchemeDefinitionBuilder{

    /// Initializes a new DigestAuthenticationSchemeDefinitionBuilder
    pub fn new() -> Self{
        Self { scheme: DigestAuthenticationSchemeDefinition::default() }
    }
    
    /// Configures the authentication scheme to load from the specified secret
    pub fn use_secret(mut self, secret: &str){
        self.scheme.use_ = Some(secret.to_string());
    }

    /// Sets the username to use
    pub fn with_username(&mut self, username: &str) -> &mut Self{
        self.scheme.username = Some(username.to_string());
        self
    }

    /// Sets the password to use
    pub fn with_password(&mut self, password: &str) -> &mut Self{
        self.scheme.password = Some(password.to_string());
        self
    }

    /// Builds the configured AuthenticationPolicyDefinition 
    pub fn build(self) -> AuthenticationPolicyDefinition{
        let mut authentication = AuthenticationPolicyDefinition::default();
        authentication.digest = Some(self.scheme);
        authentication
    }

}

/// Represents the service used to build OAuth2AuthenticationSchemeDefinitions
pub struct OAuth2AuthenticationSchemeDefinitionBuilder{
    scheme: OAuth2AuthenticationSchemeDefinition
}
impl OAuth2AuthenticationSchemeDefinitionBuilder{

    /// Initializes a new OAuth2AuthenticationSchemeDefinitions
    pub fn new() -> Self{
        Self { scheme: OAuth2AuthenticationSchemeDefinition::default() }
    }

    /// Configures the authentication scheme to load from the specified secret
    pub fn use_secret(mut self, secret: &str){
        self.scheme.use_ = Some(secret.to_string());
    }

    /// Sets the OAUTH2 endpoints to use
    pub fn with_endpoints(&mut self, endpoints: OAuth2AuthenticationEndpointsDefinition) -> &mut Self{
        self.scheme.endpoints = Some(endpoints);
        self
    }

    /// Sets the uri of the OAUTH2 authority to use
    pub fn with_authority(&mut self, uri: &str) -> &mut Self{
        self.scheme.authority = Some(uri.to_string());
        self
    }

    /// Sets the grant type to use
    pub fn with_grant_type(&mut self, grant: &str) -> &mut Self{
        self.scheme.grant = Some(grant.to_string());
        self
    }

    /// Sets the definition of the client to use
    pub fn with_client<F>(&mut self, setup: F) -> &mut Self
    where F: FnOnce(&mut OAuth2AuthenticationClientDefinitionBuilder) {
        let mut builder = OAuth2AuthenticationClientDefinitionBuilder::new();
        setup(&mut builder);
        let client = builder.build();
        self.scheme.client = Some(client);
        self
    }

    /// Sets the configuration of the request to use
    pub fn with_request<F>(&mut self, setup: F) -> &mut Self
    where F: FnOnce(&mut OAuth2AuthenticationRequestDefinitionBuilder) {
        let mut builder = OAuth2AuthenticationRequestDefinitionBuilder::new();
        setup(&mut builder);
        let request = builder.build();
        self.scheme.request = Some(request);
        self
    }
    
    /// Sets supported issuers for issued tokens
    pub fn with_issuers(&mut self, issuers: Vec<String>) -> &mut Self{
        self.scheme.issuers = Some(issuers);
        self
    }

    /// Sets the scopes to request the token for
    pub fn with_scopes(&mut self, scopes: Vec<String>) -> &mut Self{
        self.scheme.scopes = Some(scopes);
        self
    }

    /// Sets the audiences to request the token for
    pub fn with_audiences(&mut self, audiences: Vec<String>) -> &mut Self{
        self.scheme.audiences = Some(audiences);
        self
    }

    /// Sets the username to use
    pub fn with_username(&mut self, username: &str) -> &mut Self{
        self.scheme.username = Some(username.to_string());
        self
    }

    /// Sets the password to use
    pub fn with_password(&mut self, password: &str) -> &mut Self{
        self.scheme.password = Some(password.to_string());
        self
    }

    /// Sets the security token that represents the identity of the party on behalf of whom the request is being made. Used only if grant 
    pub fn with_subject(&mut self, subject: OAuth2TokenDefinition) -> &mut Self{
        self.scheme.subject = Some(subject);
        self
    }

     /// Sets the security token that represents the identity of the acting party. Typically, this will be the party that is authorized to use the requested security token and act on behalf of the subject
    pub fn with_actor(&mut self, actor: OAuth2TokenDefinition) -> &mut Self{
        self.scheme.actor = Some(actor);
        self
    }

    /// Builds the configured AuthenticationPolicyDefinition 
    pub fn build(self) -> AuthenticationPolicyDefinition{
        let mut authentication = AuthenticationPolicyDefinition::default();
        authentication.oauth2 = Some(self.scheme);
        authentication
    }

}

/// Represents the service used to build OpenIDConnectSchemeDefinition
pub struct OpenIDConnectSchemeDefinitionBuilder{
    scheme: OpenIDConnectSchemeDefinition
}
impl OpenIDConnectSchemeDefinitionBuilder{

    /// Initializes a new OpenIDConnectSchemeDefinitionBuilder
    pub fn new() -> Self{
        Self { scheme: OpenIDConnectSchemeDefinition::default() }
    }

    /// Configures the authentication scheme to load from the specified secret
    pub fn use_secret(mut self, secret: &str){
        self.scheme.use_ = Some(secret.to_string());
    }

    /// Sets the uri of the OAUTH2 authority to use
    pub fn with_authority(&mut self, uri: &str) -> &mut Self{
        self.scheme.authority = Some(uri.to_string());
        self
    }

    /// Sets the grant type to use
    pub fn with_grant_type(&mut self, grant: &str) -> &mut Self{
        self.scheme.grant = Some(grant.to_string());
        self
    }

    /// Sets the definition of the client to use
    pub fn with_client<F>(mut self, setup: F) -> Self
    where F: FnOnce(&mut OAuth2AuthenticationClientDefinitionBuilder) {
        let mut builder = OAuth2AuthenticationClientDefinitionBuilder::new();
        setup(&mut builder);
        let client = builder.build();
        self.scheme.client = Some(client);
        self
    }

    /// Sets the configuration of the request to use
    pub fn with_request<F>(&mut self, setup: F) -> &mut Self
    where F: FnOnce(&mut OAuth2AuthenticationRequestDefinitionBuilder) {
        let mut builder = OAuth2AuthenticationRequestDefinitionBuilder::new();
        setup(&mut builder);
        let request = builder.build();
        self.scheme.request = Some(request);
        self
    }
    
    /// Sets supported issuers for issued tokens
    pub fn with_issuers(&mut self, issuers: Vec<String>) -> &mut Self{
        self.scheme.issuers = Some(issuers);
        self
    }

    /// Sets the scopes to request the token for
    pub fn with_scopes(&mut self, scopes: Vec<String>) -> &mut Self{
        self.scheme.scopes = Some(scopes);
        self
    }

    /// Sets the audiences to request the token for
    pub fn with_audiences(&mut self, audiences: Vec<String>) -> &mut Self{
        self.scheme.audiences = Some(audiences);
        self
    }

    /// Sets the username to use
    pub fn with_username(&mut self, username: &str) -> &mut Self{
        self.scheme.username = Some(username.to_string());
        self
    }

    /// Sets the password to use
    pub fn with_password(&mut self, password: &str) -> &mut Self{
        self.scheme.password = Some(password.to_string());
        self
    }

    /// Sets the security token that represents the identity of the party on behalf of whom the request is being made. Used only if grant 
    pub fn with_subject(&mut self, subject: OAuth2TokenDefinition) -> &mut Self{
        self.scheme.subject = Some(subject);
        self
    }

     /// Sets the security token that represents the identity of the acting party. Typically, this will be the party that is authorized to use the requested security token and act on behalf of the subject
    pub fn with_actor(&mut self, actor: OAuth2TokenDefinition) -> &mut Self{
        self.scheme.actor = Some(actor);
        self
    }

    /// Builds the configured AuthenticationPolicyDefinition 
    pub fn build(self) -> AuthenticationPolicyDefinition{
        let mut authentication = AuthenticationPolicyDefinition::default();
        authentication.oidc = Some(self.scheme);
        authentication
    }

}

/// Represents the service used to build OAuth2AuthenticationClientDefinitions
pub struct OAuth2AuthenticationClientDefinitionBuilder{
    client: OAuth2AuthenticationClientDefinition
}
impl OAuth2AuthenticationClientDefinitionBuilder {
    
    /// Initializes a new OAuth2AuthenticationClientDefinitionBuilder
    pub fn new() -> Self{
        Self { client: OAuth2AuthenticationClientDefinition::default() }
    }

    /// Sets the OAUTH2 client's id
    pub fn with_id(&mut self, id: &str) -> &mut Self{
        self.client.id = Some(id.to_string());
        self
    }

    /// Sets the OAUTH2 client's secret
    pub fn with_secret(&mut self, secret: &str) -> &mut Self{
        self.client.secret = Some(secret.to_string());
        self
    }

    /// Sets the OAUTH2 client's assertion
    pub fn with_assertion(&mut self, assertion: &str) -> &mut Self{
        self.client.assertion = Some(assertion.to_string());
        self
    }

    /// Sets the OAUTH2 client's authentication method
    pub fn with_authentication_method(&mut self, method: &str) -> &mut Self{
        self.client.authentication = Some(method.to_string());
        self
    }

    /// Builds the configured OAuth2AuthenticationClientDefinition
    pub fn build(self) -> OAuth2AuthenticationClientDefinition{
        self.client
    }

}

/// Represents the service used to build OAuth2AuthenticationRequestDefinitions
pub struct OAuth2AuthenticationRequestDefinitionBuilder{
    request : OAuth2AuthenticationRequestDefinition
}
impl OAuth2AuthenticationRequestDefinitionBuilder {
    
    /// Initializes a new OAuth2AuthenticationRequestDefinitionBuilder
    pub fn new() -> Self{
        Self { request: OAuth2AuthenticationRequestDefinition::default() }
    }

    /// Configures the OAuth2AuthenticationRequestDefinition to build to use the specified encoding
    pub fn with_encoding(&mut self, encoding: &str) -> &mut Self{
        self.request.encoding = encoding.to_string();
        self
    }

    /// Builds the configured OAuth2AuthenticationRequestDefinition
    pub fn build(self) -> OAuth2AuthenticationRequestDefinition{
        self.request
    }

}