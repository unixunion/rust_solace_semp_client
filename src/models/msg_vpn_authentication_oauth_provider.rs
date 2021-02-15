/* 
 * SEMP (Solace Element Management Protocol)
 *
 * SEMP (starting in `v2`, see note 1) is a RESTful API for configuring, monitoring, and administering a Solace PubSub+ broker.  SEMP uses URIs to address manageable **resources** of the Solace PubSub+ broker. Resources are individual **objects**, **collections** of objects, or (exclusively in the action API) **actions**. This document applies to the following API:   API|Base Path|Purpose|Comments :---|:---|:---|:--- Configuration|/SEMP/v2/config|Reading and writing config state|See note 2    The following APIs are also available:   API|Base Path|Purpose|Comments :---|:---|:---|:--- Action|/SEMP/v2/action|Performing actions|See note 2 Monitoring|/SEMP/v2/monitor|Querying operational parameters|See note 2    Resources are always nouns, with individual objects being singular and collections being plural.  Objects within a collection are identified by an `obj-id`, which follows the collection name with the form `collection-name/obj-id`.  Actions within an object are identified by an `action-id`, which follows the object name with the form `obj-id/action-id`.  Some examples:  ``` /SEMP/v2/config/msgVpns                        ; MsgVpn collection /SEMP/v2/config/msgVpns/a                      ; MsgVpn object named \"a\" /SEMP/v2/config/msgVpns/a/queues               ; Queue collection in MsgVpn \"a\" /SEMP/v2/config/msgVpns/a/queues/b             ; Queue object named \"b\" in MsgVpn \"a\" /SEMP/v2/action/msgVpns/a/queues/b/startReplay ; Action that starts a replay on Queue \"b\" in MsgVpn \"a\" /SEMP/v2/monitor/msgVpns/a/clients             ; Client collection in MsgVpn \"a\" /SEMP/v2/monitor/msgVpns/a/clients/c           ; Client object named \"c\" in MsgVpn \"a\" ```  ## Collection Resources  Collections are unordered lists of objects (unless described as otherwise), and are described by JSON arrays. Each item in the array represents an object in the same manner as the individual object would normally be represented. In the configuration API, the creation of a new object is done through its collection resource.  ## Object and Action Resources  Objects are composed of attributes, actions, collections, and other objects. They are described by JSON objects as name/value pairs. The collections and actions of an object are not contained directly in the object's JSON content; rather the content includes an attribute containing a URI which points to the collections and actions. These contained resources must be managed through this URI. At a minimum, every object has one or more identifying attributes, and its own `uri` attribute which contains the URI pointing to itself.  Actions are also composed of attributes, and are described by JSON objects as name/value pairs. Unlike objects, however, they are not members of a collection and cannot be retrieved, only performed. Actions only exist in the action API.  Attributes in an object or action may have any combination of the following properties:   Property|Meaning|Comments :---|:---|:--- Identifying|Attribute is involved in unique identification of the object, and appears in its URI| Required|Attribute must be provided in the request| Read-Only|Attribute can only be read, not written.|See note 3 Write-Only|Attribute can only be written, not read, unless the attribute is also opaque|See the documentation for the opaque property Requires-Disable|Attribute can only be changed when object is disabled| Deprecated|Attribute is deprecated, and will disappear in the next SEMP version| Opaque|Attribute can be set or retrieved in opaque form when the `opaquePassword` query parameter is present|See the `opaquePassword` query parameter documentation    In some requests, certain attributes may only be provided in certain combinations with other attributes:   Relationship|Meaning :---|:--- Requires|Attribute may only be changed by a request if a particular attribute or combination of attributes is also provided in the request Conflicts|Attribute may only be provided in a request if a particular attribute or combination of attributes is not also provided in the request    In the monitoring API, any non-identifying attribute may not be returned in a GET.  ## HTTP Methods  The following HTTP methods manipulate resources in accordance with these general principles. Note that some methods are only used in certain APIs:   Method|Resource|Meaning|Request Body|Response Body|Missing Request Attributes :---|:---|:---|:---|:---|:--- POST|Collection|Create object|Initial attribute values|Object attributes and metadata|Set to default PUT|Object|Create or replace object (see note 5)|New attribute values|Object attributes and metadata|Set to default, with certain exceptions (see note 4) PUT|Action|Performs action|Action arguments|Action metadata|N/A PATCH|Object|Update object|New attribute values|Object attributes and metadata|unchanged DELETE|Object|Delete object|Empty|Object metadata|N/A GET|Object|Get object|Empty|Object attributes and metadata|N/A GET|Collection|Get collection|Empty|Object attributes and collection metadata|N/A    ## Common Query Parameters  The following are some common query parameters that are supported by many method/URI combinations. Individual URIs may document additional parameters. Note that multiple query parameters can be used together in a single URI, separated by the ampersand character. For example:  ``` ; Request for the MsgVpns collection using two hypothetical query parameters ; \"q1\" and \"q2\" with values \"val1\" and \"val2\" respectively /SEMP/v2/config/msgVpns?q1=val1&q2=val2 ```  ### select  Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. Use this query parameter to limit the size of the returned data for each returned object, return only those fields that are desired, or exclude fields that are not desired.  The value of `select` is a comma-separated list of attribute names. If the list contains attribute names that are not prefaced by `-`, only those attributes are included in the response. If the list contains attribute names that are prefaced by `-`, those attributes are excluded from the response. If the list contains both types, then the difference of the first set of attributes and the second set of attributes is returned. If the list is empty (i.e. `select=`), no attributes are returned.  All attributes that are prefaced by `-` must follow all attributes that are not prefaced by `-`. In addition, each attribute name in the list must match at least one attribute in the object.  Names may include the `*` wildcard (zero or more characters). Nested attribute names are supported using periods (e.g. `parentName.childName`).  Some examples:  ``` ; List of all MsgVpn names /SEMP/v2/config/msgVpns?select=msgVpnName ; List of all MsgVpn and their attributes except for their names /SEMP/v2/config/msgVpns?select=-msgVpnName ; Authentication attributes of MsgVpn \"finance\" /SEMP/v2/config/msgVpns/finance?select=authentication* ; All attributes of MsgVpn \"finance\" except for authentication attributes /SEMP/v2/config/msgVpns/finance?select=-authentication* ; Access related attributes of Queue \"orderQ\" of MsgVpn \"finance\" /SEMP/v2/config/msgVpns/finance/queues/orderQ?select=owner,permission ```  ### where  Include in the response only objects where certain conditions are true. Use this query parameter to limit which objects are returned to those whose attribute values meet the given conditions.  The value of `where` is a comma-separated list of expressions. All expressions must be true for the object to be included in the response. Each expression takes the form:  ``` expression  = attribute-name OP value OP          = '==' | '!=' | '&lt;' | '&gt;' | '&lt;=' | '&gt;=' ```  `value` may be a number, string, `true`, or `false`, as appropriate for the type of `attribute-name`. Greater-than and less-than comparisons only work for numbers. A `*` in a string `value` is interpreted as a wildcard (zero or more characters). Some examples:  ``` ; Only enabled MsgVpns /SEMP/v2/config/msgVpns?where=enabled==true ; Only MsgVpns using basic non-LDAP authentication /SEMP/v2/config/msgVpns?where=authenticationBasicEnabled==true,authenticationBasicType!=ldap ; Only MsgVpns that allow more than 100 client connections /SEMP/v2/config/msgVpns?where=maxConnectionCount>100 ; Only MsgVpns with msgVpnName starting with \"B\": /SEMP/v2/config/msgVpns?where=msgVpnName==B* ```  ### count  Limit the count of objects in the response. This can be useful to limit the size of the response for large collections. The minimum value for `count` is `1` and the default is `10`. There is also a per-collection maximum value to limit request handling time. For example:  ``` ; Up to 25 MsgVpns /SEMP/v2/config/msgVpns?count=25 ```  ### cursor  The cursor, or position, for the next page of objects. Cursors are opaque data that should not be created or interpreted by SEMP clients, and should only be used as described below.  When a request is made for a collection and there may be additional objects available for retrieval that are not included in the initial response, the response will include a `cursorQuery` field containing a cursor. The value of this field can be specified in the `cursor` query parameter of a subsequent request to retrieve the next page of objects. For convenience, an appropriate URI is constructed automatically by the broker and included in the `nextPageUri` field of the response. This URI can be used directly to retrieve the next page of objects.  ### opaquePassword  Attributes with the opaque property are also write-only and so cannot normally be retrieved in a GET. However, when a password is provided in the `opaquePassword` query parameter, attributes with the opaque property are retrieved in a GET in opaque form, encrypted with this password. The query parameter can also be used on a POST, PATCH, or PUT to set opaque attributes using opaque attribute values retrieved in a GET, so long as:  1. the same password that was used to retrieve the opaque attribute values is provided; and  2. the broker to which the request is being sent has the same major and minor SEMP version as the broker that produced the opaque attribute values.  The password provided in the query parameter must be a minimum of 8 characters and a maximum of 128 characters.  The query parameter can only be used in the configuration API, and only over HTTPS.  ## Help  Visit [our website](https://solace.com) to learn more about Solace.  You can also download the SEMP API specifications by clicking [here](https://solace.com/downloads/).  If you need additional support, please contact us at [support@solace.com](mailto:support@solace.com).  ## Notes  Note|Description :---:|:--- 1|This specification defines SEMP starting in \"v2\", and not the original SEMP \"v1\" interface. Request and response formats between \"v1\" and \"v2\" are entirely incompatible, although both protocols share a common port configuration on the Solace PubSub+ broker. They are differentiated by the initial portion of the URI path, one of either \"/SEMP/\" or \"/SEMP/v2/\" 2|This API is partially implemented. Only a subset of all objects are available. 3|Read-only attributes may appear in POST and PUT/PATCH requests. However, if a read-only attribute is not marked as identifying, it will be ignored during a PUT/PATCH. 4|On a PUT, if the SEMP user is not authorized to modify the attribute, its value is left unchanged rather than set to default. In addition, the values of write-only attributes are not set to their defaults on a PUT, except in the following two cases: there is a mutual requires relationship with another non-write-only attribute and both attributes are absent from the request; or the attribute is also opaque and the `opaquePassword` query parameter is provided in the request. 5|On a PUT, if the object does not exist, it is created first.  
 *
 * OpenAPI spec version: 2.19
 * Contact: support@solace.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MsgVpnAuthenticationOauthProvider {
  /// The audience claim name, indicating which part of the object to use for determining the audience. The default value is `\"aud\"`.
  #[serde(rename = "audienceClaimName", skip_serializing_if="Option::is_none")]
  audience_claim_name: Option<String>,
  /// The audience claim source, indicating where to search for the audience value. The default value is `\"id-token\"`. The allowed values and their meaning are:  <pre> \"access-token\" - The OAuth v2 access_token. \"id-token\" - The OpenID Connect id_token. \"introspection\" - The result of introspecting the OAuth v2 access_token. </pre> 
  #[serde(rename = "audienceClaimSource", skip_serializing_if="Option::is_none")]
  audience_claim_source: Option<String>,
  /// The required audience value for a token to be considered valid. The default value is `\"\"`.
  #[serde(rename = "audienceClaimValue", skip_serializing_if="Option::is_none")]
  audience_claim_value: Option<String>,
  /// Enable or disable audience validation. The default value is `false`.
  #[serde(rename = "audienceValidationEnabled", skip_serializing_if="Option::is_none")]
  audience_validation_enabled: Option<bool>,
  /// The authorization group claim name, indicating which part of the object to use for determining the authorization group. The default value is `\"scope\"`.
  #[serde(rename = "authorizationGroupClaimName", skip_serializing_if="Option::is_none")]
  authorization_group_claim_name: Option<String>,
  /// The authorization group claim source, indicating where to search for the authorization group name. The default value is `\"id-token\"`. The allowed values and their meaning are:  <pre> \"access-token\" - The OAuth v2 access_token. \"id-token\" - The OpenID Connect id_token. \"introspection\" - The result of introspecting the OAuth v2 access_token. </pre> 
  #[serde(rename = "authorizationGroupClaimSource", skip_serializing_if="Option::is_none")]
  authorization_group_claim_source: Option<String>,
  /// Enable or disable OAuth based authorization. When enabled, the configured authorization type for OAuth clients is overridden. The default value is `false`.
  #[serde(rename = "authorizationGroupEnabled", skip_serializing_if="Option::is_none")]
  authorization_group_enabled: Option<bool>,
  /// Enable or disable the disconnection of clients when their tokens expire. Changing this value does not affect existing clients, only new client connections. The default value is `true`.
  #[serde(rename = "disconnectOnTokenExpirationEnabled", skip_serializing_if="Option::is_none")]
  disconnect_on_token_expiration_enabled: Option<bool>,
  /// Enable or disable OAuth Provider client authentication. The default value is `false`.
  #[serde(rename = "enabled", skip_serializing_if="Option::is_none")]
  enabled: Option<bool>,
  /// The number of seconds between forced JWKS public key refreshing. The default value is `86400`.
  #[serde(rename = "jwksRefreshInterval", skip_serializing_if="Option::is_none")]
  jwks_refresh_interval: Option<i32>,
  /// The URI where the OAuth provider publishes its JWKS public keys. The default value is `\"\"`.
  #[serde(rename = "jwksUri", skip_serializing_if="Option::is_none")]
  jwks_uri: Option<String>,
  /// The name of the Message VPN.
  #[serde(rename = "msgVpnName", skip_serializing_if="Option::is_none")]
  msg_vpn_name: Option<String>,
  /// The name of the OAuth Provider.
  #[serde(rename = "oauthProviderName", skip_serializing_if="Option::is_none")]
  oauth_provider_name: Option<String>,
  /// Enable or disable whether to ignore time limits and accept tokens that are not yet valid or are no longer valid. The default value is `false`.
  #[serde(rename = "tokenIgnoreTimeLimitsEnabled", skip_serializing_if="Option::is_none")]
  token_ignore_time_limits_enabled: Option<bool>,
  /// The parameter name used to identify the token during access token introspection. A standards compliant OAuth introspection server expects \"token\". The default value is `\"token\"`.
  #[serde(rename = "tokenIntrospectionParameterName", skip_serializing_if="Option::is_none")]
  token_introspection_parameter_name: Option<String>,
  /// The password to use when logging into the token introspection URI. This attribute is absent from a GET and not updated when absent in a PUT, subject to the exceptions in note 4. The default value is `\"\"`.
  #[serde(rename = "tokenIntrospectionPassword", skip_serializing_if="Option::is_none")]
  token_introspection_password: Option<String>,
  /// The maximum time in seconds a token introspection is allowed to take. The default value is `1`.
  #[serde(rename = "tokenIntrospectionTimeout", skip_serializing_if="Option::is_none")]
  token_introspection_timeout: Option<i32>,
  /// The token introspection URI of the OAuth authentication server. The default value is `\"\"`.
  #[serde(rename = "tokenIntrospectionUri", skip_serializing_if="Option::is_none")]
  token_introspection_uri: Option<String>,
  /// The username to use when logging into the token introspection URI. The default value is `\"\"`.
  #[serde(rename = "tokenIntrospectionUsername", skip_serializing_if="Option::is_none")]
  token_introspection_username: Option<String>,
  /// The username claim name, indicating which part of the object to use for determining the username. The default value is `\"sub\"`.
  #[serde(rename = "usernameClaimName", skip_serializing_if="Option::is_none")]
  username_claim_name: Option<String>,
  /// The username claim source, indicating where to search for the username value. The default value is `\"id-token\"`. The allowed values and their meaning are:  <pre> \"access-token\" - The OAuth v2 access_token. \"id-token\" - The OpenID Connect id_token. \"introspection\" - The result of introspecting the OAuth v2 access_token. </pre> 
  #[serde(rename = "usernameClaimSource", skip_serializing_if="Option::is_none")]
  username_claim_source: Option<String>,
  /// Enable or disable whether the API provided username will be validated against the username calculated from the token(s); the connection attempt is rejected if they differ. The default value is `false`.
  #[serde(rename = "usernameValidateEnabled", skip_serializing_if="Option::is_none")]
  username_validate_enabled: Option<bool>
}

impl MsgVpnAuthenticationOauthProvider {
  pub fn new() -> MsgVpnAuthenticationOauthProvider {
    MsgVpnAuthenticationOauthProvider {
      audience_claim_name: None,
      audience_claim_source: None,
      audience_claim_value: None,
      audience_validation_enabled: None,
      authorization_group_claim_name: None,
      authorization_group_claim_source: None,
      authorization_group_enabled: None,
      disconnect_on_token_expiration_enabled: None,
      enabled: None,
      jwks_refresh_interval: None,
      jwks_uri: None,
      msg_vpn_name: None,
      oauth_provider_name: None,
      token_ignore_time_limits_enabled: None,
      token_introspection_parameter_name: None,
      token_introspection_password: None,
      token_introspection_timeout: None,
      token_introspection_uri: None,
      token_introspection_username: None,
      username_claim_name: None,
      username_claim_source: None,
      username_validate_enabled: None
    }
  }

  pub fn set_audience_claim_name(&mut self, audience_claim_name: String) {
    self.audience_claim_name = Some(audience_claim_name);
  }

  pub fn with_audience_claim_name(mut self, audience_claim_name: String) -> MsgVpnAuthenticationOauthProvider {
    self.audience_claim_name = Some(audience_claim_name);
    self
  }

  pub fn audience_claim_name(&self) -> Option<&String> {
    self.audience_claim_name.as_ref()
  }

  pub fn reset_audience_claim_name(&mut self) {
    self.audience_claim_name = None;
  }

  pub fn set_audience_claim_source(&mut self, audience_claim_source: String) {
    self.audience_claim_source = Some(audience_claim_source);
  }

  pub fn with_audience_claim_source(mut self, audience_claim_source: String) -> MsgVpnAuthenticationOauthProvider {
    self.audience_claim_source = Some(audience_claim_source);
    self
  }

  pub fn audience_claim_source(&self) -> Option<&String> {
    self.audience_claim_source.as_ref()
  }

  pub fn reset_audience_claim_source(&mut self) {
    self.audience_claim_source = None;
  }

  pub fn set_audience_claim_value(&mut self, audience_claim_value: String) {
    self.audience_claim_value = Some(audience_claim_value);
  }

  pub fn with_audience_claim_value(mut self, audience_claim_value: String) -> MsgVpnAuthenticationOauthProvider {
    self.audience_claim_value = Some(audience_claim_value);
    self
  }

  pub fn audience_claim_value(&self) -> Option<&String> {
    self.audience_claim_value.as_ref()
  }

  pub fn reset_audience_claim_value(&mut self) {
    self.audience_claim_value = None;
  }

  pub fn set_audience_validation_enabled(&mut self, audience_validation_enabled: bool) {
    self.audience_validation_enabled = Some(audience_validation_enabled);
  }

  pub fn with_audience_validation_enabled(mut self, audience_validation_enabled: bool) -> MsgVpnAuthenticationOauthProvider {
    self.audience_validation_enabled = Some(audience_validation_enabled);
    self
  }

  pub fn audience_validation_enabled(&self) -> Option<&bool> {
    self.audience_validation_enabled.as_ref()
  }

  pub fn reset_audience_validation_enabled(&mut self) {
    self.audience_validation_enabled = None;
  }

  pub fn set_authorization_group_claim_name(&mut self, authorization_group_claim_name: String) {
    self.authorization_group_claim_name = Some(authorization_group_claim_name);
  }

  pub fn with_authorization_group_claim_name(mut self, authorization_group_claim_name: String) -> MsgVpnAuthenticationOauthProvider {
    self.authorization_group_claim_name = Some(authorization_group_claim_name);
    self
  }

  pub fn authorization_group_claim_name(&self) -> Option<&String> {
    self.authorization_group_claim_name.as_ref()
  }

  pub fn reset_authorization_group_claim_name(&mut self) {
    self.authorization_group_claim_name = None;
  }

  pub fn set_authorization_group_claim_source(&mut self, authorization_group_claim_source: String) {
    self.authorization_group_claim_source = Some(authorization_group_claim_source);
  }

  pub fn with_authorization_group_claim_source(mut self, authorization_group_claim_source: String) -> MsgVpnAuthenticationOauthProvider {
    self.authorization_group_claim_source = Some(authorization_group_claim_source);
    self
  }

  pub fn authorization_group_claim_source(&self) -> Option<&String> {
    self.authorization_group_claim_source.as_ref()
  }

  pub fn reset_authorization_group_claim_source(&mut self) {
    self.authorization_group_claim_source = None;
  }

  pub fn set_authorization_group_enabled(&mut self, authorization_group_enabled: bool) {
    self.authorization_group_enabled = Some(authorization_group_enabled);
  }

  pub fn with_authorization_group_enabled(mut self, authorization_group_enabled: bool) -> MsgVpnAuthenticationOauthProvider {
    self.authorization_group_enabled = Some(authorization_group_enabled);
    self
  }

  pub fn authorization_group_enabled(&self) -> Option<&bool> {
    self.authorization_group_enabled.as_ref()
  }

  pub fn reset_authorization_group_enabled(&mut self) {
    self.authorization_group_enabled = None;
  }

  pub fn set_disconnect_on_token_expiration_enabled(&mut self, disconnect_on_token_expiration_enabled: bool) {
    self.disconnect_on_token_expiration_enabled = Some(disconnect_on_token_expiration_enabled);
  }

  pub fn with_disconnect_on_token_expiration_enabled(mut self, disconnect_on_token_expiration_enabled: bool) -> MsgVpnAuthenticationOauthProvider {
    self.disconnect_on_token_expiration_enabled = Some(disconnect_on_token_expiration_enabled);
    self
  }

  pub fn disconnect_on_token_expiration_enabled(&self) -> Option<&bool> {
    self.disconnect_on_token_expiration_enabled.as_ref()
  }

  pub fn reset_disconnect_on_token_expiration_enabled(&mut self) {
    self.disconnect_on_token_expiration_enabled = None;
  }

  pub fn set_enabled(&mut self, enabled: bool) {
    self.enabled = Some(enabled);
  }

  pub fn with_enabled(mut self, enabled: bool) -> MsgVpnAuthenticationOauthProvider {
    self.enabled = Some(enabled);
    self
  }

  pub fn enabled(&self) -> Option<&bool> {
    self.enabled.as_ref()
  }

  pub fn reset_enabled(&mut self) {
    self.enabled = None;
  }

  pub fn set_jwks_refresh_interval(&mut self, jwks_refresh_interval: i32) {
    self.jwks_refresh_interval = Some(jwks_refresh_interval);
  }

  pub fn with_jwks_refresh_interval(mut self, jwks_refresh_interval: i32) -> MsgVpnAuthenticationOauthProvider {
    self.jwks_refresh_interval = Some(jwks_refresh_interval);
    self
  }

  pub fn jwks_refresh_interval(&self) -> Option<&i32> {
    self.jwks_refresh_interval.as_ref()
  }

  pub fn reset_jwks_refresh_interval(&mut self) {
    self.jwks_refresh_interval = None;
  }

  pub fn set_jwks_uri(&mut self, jwks_uri: String) {
    self.jwks_uri = Some(jwks_uri);
  }

  pub fn with_jwks_uri(mut self, jwks_uri: String) -> MsgVpnAuthenticationOauthProvider {
    self.jwks_uri = Some(jwks_uri);
    self
  }

  pub fn jwks_uri(&self) -> Option<&String> {
    self.jwks_uri.as_ref()
  }

  pub fn reset_jwks_uri(&mut self) {
    self.jwks_uri = None;
  }

  pub fn set_msg_vpn_name(&mut self, msg_vpn_name: String) {
    self.msg_vpn_name = Some(msg_vpn_name);
  }

  pub fn with_msg_vpn_name(mut self, msg_vpn_name: String) -> MsgVpnAuthenticationOauthProvider {
    self.msg_vpn_name = Some(msg_vpn_name);
    self
  }

  pub fn msg_vpn_name(&self) -> Option<&String> {
    self.msg_vpn_name.as_ref()
  }

  pub fn reset_msg_vpn_name(&mut self) {
    self.msg_vpn_name = None;
  }

  pub fn set_oauth_provider_name(&mut self, oauth_provider_name: String) {
    self.oauth_provider_name = Some(oauth_provider_name);
  }

  pub fn with_oauth_provider_name(mut self, oauth_provider_name: String) -> MsgVpnAuthenticationOauthProvider {
    self.oauth_provider_name = Some(oauth_provider_name);
    self
  }

  pub fn oauth_provider_name(&self) -> Option<&String> {
    self.oauth_provider_name.as_ref()
  }

  pub fn reset_oauth_provider_name(&mut self) {
    self.oauth_provider_name = None;
  }

  pub fn set_token_ignore_time_limits_enabled(&mut self, token_ignore_time_limits_enabled: bool) {
    self.token_ignore_time_limits_enabled = Some(token_ignore_time_limits_enabled);
  }

  pub fn with_token_ignore_time_limits_enabled(mut self, token_ignore_time_limits_enabled: bool) -> MsgVpnAuthenticationOauthProvider {
    self.token_ignore_time_limits_enabled = Some(token_ignore_time_limits_enabled);
    self
  }

  pub fn token_ignore_time_limits_enabled(&self) -> Option<&bool> {
    self.token_ignore_time_limits_enabled.as_ref()
  }

  pub fn reset_token_ignore_time_limits_enabled(&mut self) {
    self.token_ignore_time_limits_enabled = None;
  }

  pub fn set_token_introspection_parameter_name(&mut self, token_introspection_parameter_name: String) {
    self.token_introspection_parameter_name = Some(token_introspection_parameter_name);
  }

  pub fn with_token_introspection_parameter_name(mut self, token_introspection_parameter_name: String) -> MsgVpnAuthenticationOauthProvider {
    self.token_introspection_parameter_name = Some(token_introspection_parameter_name);
    self
  }

  pub fn token_introspection_parameter_name(&self) -> Option<&String> {
    self.token_introspection_parameter_name.as_ref()
  }

  pub fn reset_token_introspection_parameter_name(&mut self) {
    self.token_introspection_parameter_name = None;
  }

  pub fn set_token_introspection_password(&mut self, token_introspection_password: String) {
    self.token_introspection_password = Some(token_introspection_password);
  }

  pub fn with_token_introspection_password(mut self, token_introspection_password: String) -> MsgVpnAuthenticationOauthProvider {
    self.token_introspection_password = Some(token_introspection_password);
    self
  }

  pub fn token_introspection_password(&self) -> Option<&String> {
    self.token_introspection_password.as_ref()
  }

  pub fn reset_token_introspection_password(&mut self) {
    self.token_introspection_password = None;
  }

  pub fn set_token_introspection_timeout(&mut self, token_introspection_timeout: i32) {
    self.token_introspection_timeout = Some(token_introspection_timeout);
  }

  pub fn with_token_introspection_timeout(mut self, token_introspection_timeout: i32) -> MsgVpnAuthenticationOauthProvider {
    self.token_introspection_timeout = Some(token_introspection_timeout);
    self
  }

  pub fn token_introspection_timeout(&self) -> Option<&i32> {
    self.token_introspection_timeout.as_ref()
  }

  pub fn reset_token_introspection_timeout(&mut self) {
    self.token_introspection_timeout = None;
  }

  pub fn set_token_introspection_uri(&mut self, token_introspection_uri: String) {
    self.token_introspection_uri = Some(token_introspection_uri);
  }

  pub fn with_token_introspection_uri(mut self, token_introspection_uri: String) -> MsgVpnAuthenticationOauthProvider {
    self.token_introspection_uri = Some(token_introspection_uri);
    self
  }

  pub fn token_introspection_uri(&self) -> Option<&String> {
    self.token_introspection_uri.as_ref()
  }

  pub fn reset_token_introspection_uri(&mut self) {
    self.token_introspection_uri = None;
  }

  pub fn set_token_introspection_username(&mut self, token_introspection_username: String) {
    self.token_introspection_username = Some(token_introspection_username);
  }

  pub fn with_token_introspection_username(mut self, token_introspection_username: String) -> MsgVpnAuthenticationOauthProvider {
    self.token_introspection_username = Some(token_introspection_username);
    self
  }

  pub fn token_introspection_username(&self) -> Option<&String> {
    self.token_introspection_username.as_ref()
  }

  pub fn reset_token_introspection_username(&mut self) {
    self.token_introspection_username = None;
  }

  pub fn set_username_claim_name(&mut self, username_claim_name: String) {
    self.username_claim_name = Some(username_claim_name);
  }

  pub fn with_username_claim_name(mut self, username_claim_name: String) -> MsgVpnAuthenticationOauthProvider {
    self.username_claim_name = Some(username_claim_name);
    self
  }

  pub fn username_claim_name(&self) -> Option<&String> {
    self.username_claim_name.as_ref()
  }

  pub fn reset_username_claim_name(&mut self) {
    self.username_claim_name = None;
  }

  pub fn set_username_claim_source(&mut self, username_claim_source: String) {
    self.username_claim_source = Some(username_claim_source);
  }

  pub fn with_username_claim_source(mut self, username_claim_source: String) -> MsgVpnAuthenticationOauthProvider {
    self.username_claim_source = Some(username_claim_source);
    self
  }

  pub fn username_claim_source(&self) -> Option<&String> {
    self.username_claim_source.as_ref()
  }

  pub fn reset_username_claim_source(&mut self) {
    self.username_claim_source = None;
  }

  pub fn set_username_validate_enabled(&mut self, username_validate_enabled: bool) {
    self.username_validate_enabled = Some(username_validate_enabled);
  }

  pub fn with_username_validate_enabled(mut self, username_validate_enabled: bool) -> MsgVpnAuthenticationOauthProvider {
    self.username_validate_enabled = Some(username_validate_enabled);
    self
  }

  pub fn username_validate_enabled(&self) -> Option<&bool> {
    self.username_validate_enabled.as_ref()
  }

  pub fn reset_username_validate_enabled(&mut self) {
    self.username_validate_enabled = None;
  }

}



