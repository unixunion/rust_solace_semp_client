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
pub struct MsgVpnBridge {
  /// The name of the Bridge.
  #[serde(rename = "bridgeName", skip_serializing_if="Option::is_none")]
  bridge_name: Option<String>,
  /// The virtual router of the Bridge. The allowed values and their meaning are:  <pre> \"primary\" - The Bridge is used for the primary virtual router. \"backup\" - The Bridge is used for the backup virtual router. \"auto\" - The Bridge is automatically assigned a virtual router at creation, depending on the broker's active-standby role. </pre> 
  #[serde(rename = "bridgeVirtualRouter", skip_serializing_if="Option::is_none")]
  bridge_virtual_router: Option<String>,
  /// Enable or disable the Bridge. The default value is `false`.
  #[serde(rename = "enabled", skip_serializing_if="Option::is_none")]
  enabled: Option<bool>,
  /// The maximum time-to-live (TTL) in hops. Messages are discarded if their TTL exceeds this value. The default value is `8`.
  #[serde(rename = "maxTtl", skip_serializing_if="Option::is_none")]
  max_ttl: Option<i64>,
  /// The name of the Message VPN.
  #[serde(rename = "msgVpnName", skip_serializing_if="Option::is_none")]
  msg_vpn_name: Option<String>,
  /// The Client Username the Bridge uses to login to the remote Message VPN. The default value is `\"\"`.
  #[serde(rename = "remoteAuthenticationBasicClientUsername", skip_serializing_if="Option::is_none")]
  remote_authentication_basic_client_username: Option<String>,
  /// The password for the Client Username. This attribute is absent from a GET and not updated when absent in a PUT, subject to the exceptions in note 4. The default value is `\"\"`.
  #[serde(rename = "remoteAuthenticationBasicPassword", skip_serializing_if="Option::is_none")]
  remote_authentication_basic_password: Option<String>,
  /// The PEM formatted content for the client certificate used by the Bridge to login to the remote Message VPN. It must consist of a private key and between one and three certificates comprising the certificate trust chain. This attribute is absent from a GET and not updated when absent in a PUT, subject to the exceptions in note 4. Changing this attribute requires an HTTPS connection. The default value is `\"\"`. Available since 2.9.
  #[serde(rename = "remoteAuthenticationClientCertContent", skip_serializing_if="Option::is_none")]
  remote_authentication_client_cert_content: Option<String>,
  /// The password for the client certificate. This attribute is absent from a GET and not updated when absent in a PUT, subject to the exceptions in note 4. Changing this attribute requires an HTTPS connection. The default value is `\"\"`. Available since 2.9.
  #[serde(rename = "remoteAuthenticationClientCertPassword", skip_serializing_if="Option::is_none")]
  remote_authentication_client_cert_password: Option<String>,
  /// The authentication scheme for the remote Message VPN. The default value is `\"basic\"`. The allowed values and their meaning are:  <pre> \"basic\" - Basic Authentication Scheme (via username and password). \"client-certificate\" - Client Certificate Authentication Scheme (via certificate file or content). </pre> 
  #[serde(rename = "remoteAuthenticationScheme", skip_serializing_if="Option::is_none")]
  remote_authentication_scheme: Option<String>,
  /// The maximum number of retry attempts to establish a connection to the remote Message VPN. A value of 0 means to retry forever. The default value is `0`.
  #[serde(rename = "remoteConnectionRetryCount", skip_serializing_if="Option::is_none")]
  remote_connection_retry_count: Option<i64>,
  /// The number of seconds the broker waits for the bridge connection to be established before attempting a new connection. The default value is `3`.
  #[serde(rename = "remoteConnectionRetryDelay", skip_serializing_if="Option::is_none")]
  remote_connection_retry_delay: Option<i64>,
  /// The priority for deliver-to-one (DTO) messages transmitted from the remote Message VPN. The default value is `\"p1\"`. The allowed values and their meaning are:  <pre> \"p1\" - The 1st or highest priority. \"p2\" - The 2nd highest priority. \"p3\" - The 3rd highest priority. \"p4\" - The 4th highest priority. \"da\" - Ignore priority and deliver always. </pre> 
  #[serde(rename = "remoteDeliverToOnePriority", skip_serializing_if="Option::is_none")]
  remote_deliver_to_one_priority: Option<String>,
  /// The colon-separated list of cipher suites supported for TLS connections to the remote Message VPN. The value \"default\" implies all supported suites ordered from most secure to least secure. The default value is `\"default\"`.
  #[serde(rename = "tlsCipherSuiteList", skip_serializing_if="Option::is_none")]
  tls_cipher_suite_list: Option<String>
}

impl MsgVpnBridge {
  pub fn new() -> MsgVpnBridge {
    MsgVpnBridge {
      bridge_name: None,
      bridge_virtual_router: None,
      enabled: None,
      max_ttl: None,
      msg_vpn_name: None,
      remote_authentication_basic_client_username: None,
      remote_authentication_basic_password: None,
      remote_authentication_client_cert_content: None,
      remote_authentication_client_cert_password: None,
      remote_authentication_scheme: None,
      remote_connection_retry_count: None,
      remote_connection_retry_delay: None,
      remote_deliver_to_one_priority: None,
      tls_cipher_suite_list: None
    }
  }

  pub fn set_bridge_name(&mut self, bridge_name: String) {
    self.bridge_name = Some(bridge_name);
  }

  pub fn with_bridge_name(mut self, bridge_name: String) -> MsgVpnBridge {
    self.bridge_name = Some(bridge_name);
    self
  }

  pub fn bridge_name(&self) -> Option<&String> {
    self.bridge_name.as_ref()
  }

  pub fn reset_bridge_name(&mut self) {
    self.bridge_name = None;
  }

  pub fn set_bridge_virtual_router(&mut self, bridge_virtual_router: String) {
    self.bridge_virtual_router = Some(bridge_virtual_router);
  }

  pub fn with_bridge_virtual_router(mut self, bridge_virtual_router: String) -> MsgVpnBridge {
    self.bridge_virtual_router = Some(bridge_virtual_router);
    self
  }

  pub fn bridge_virtual_router(&self) -> Option<&String> {
    self.bridge_virtual_router.as_ref()
  }

  pub fn reset_bridge_virtual_router(&mut self) {
    self.bridge_virtual_router = None;
  }

  pub fn set_enabled(&mut self, enabled: bool) {
    self.enabled = Some(enabled);
  }

  pub fn with_enabled(mut self, enabled: bool) -> MsgVpnBridge {
    self.enabled = Some(enabled);
    self
  }

  pub fn enabled(&self) -> Option<&bool> {
    self.enabled.as_ref()
  }

  pub fn reset_enabled(&mut self) {
    self.enabled = None;
  }

  pub fn set_max_ttl(&mut self, max_ttl: i64) {
    self.max_ttl = Some(max_ttl);
  }

  pub fn with_max_ttl(mut self, max_ttl: i64) -> MsgVpnBridge {
    self.max_ttl = Some(max_ttl);
    self
  }

  pub fn max_ttl(&self) -> Option<&i64> {
    self.max_ttl.as_ref()
  }

  pub fn reset_max_ttl(&mut self) {
    self.max_ttl = None;
  }

  pub fn set_msg_vpn_name(&mut self, msg_vpn_name: String) {
    self.msg_vpn_name = Some(msg_vpn_name);
  }

  pub fn with_msg_vpn_name(mut self, msg_vpn_name: String) -> MsgVpnBridge {
    self.msg_vpn_name = Some(msg_vpn_name);
    self
  }

  pub fn msg_vpn_name(&self) -> Option<&String> {
    self.msg_vpn_name.as_ref()
  }

  pub fn reset_msg_vpn_name(&mut self) {
    self.msg_vpn_name = None;
  }

  pub fn set_remote_authentication_basic_client_username(&mut self, remote_authentication_basic_client_username: String) {
    self.remote_authentication_basic_client_username = Some(remote_authentication_basic_client_username);
  }

  pub fn with_remote_authentication_basic_client_username(mut self, remote_authentication_basic_client_username: String) -> MsgVpnBridge {
    self.remote_authentication_basic_client_username = Some(remote_authentication_basic_client_username);
    self
  }

  pub fn remote_authentication_basic_client_username(&self) -> Option<&String> {
    self.remote_authentication_basic_client_username.as_ref()
  }

  pub fn reset_remote_authentication_basic_client_username(&mut self) {
    self.remote_authentication_basic_client_username = None;
  }

  pub fn set_remote_authentication_basic_password(&mut self, remote_authentication_basic_password: String) {
    self.remote_authentication_basic_password = Some(remote_authentication_basic_password);
  }

  pub fn with_remote_authentication_basic_password(mut self, remote_authentication_basic_password: String) -> MsgVpnBridge {
    self.remote_authentication_basic_password = Some(remote_authentication_basic_password);
    self
  }

  pub fn remote_authentication_basic_password(&self) -> Option<&String> {
    self.remote_authentication_basic_password.as_ref()
  }

  pub fn reset_remote_authentication_basic_password(&mut self) {
    self.remote_authentication_basic_password = None;
  }

  pub fn set_remote_authentication_client_cert_content(&mut self, remote_authentication_client_cert_content: String) {
    self.remote_authentication_client_cert_content = Some(remote_authentication_client_cert_content);
  }

  pub fn with_remote_authentication_client_cert_content(mut self, remote_authentication_client_cert_content: String) -> MsgVpnBridge {
    self.remote_authentication_client_cert_content = Some(remote_authentication_client_cert_content);
    self
  }

  pub fn remote_authentication_client_cert_content(&self) -> Option<&String> {
    self.remote_authentication_client_cert_content.as_ref()
  }

  pub fn reset_remote_authentication_client_cert_content(&mut self) {
    self.remote_authentication_client_cert_content = None;
  }

  pub fn set_remote_authentication_client_cert_password(&mut self, remote_authentication_client_cert_password: String) {
    self.remote_authentication_client_cert_password = Some(remote_authentication_client_cert_password);
  }

  pub fn with_remote_authentication_client_cert_password(mut self, remote_authentication_client_cert_password: String) -> MsgVpnBridge {
    self.remote_authentication_client_cert_password = Some(remote_authentication_client_cert_password);
    self
  }

  pub fn remote_authentication_client_cert_password(&self) -> Option<&String> {
    self.remote_authentication_client_cert_password.as_ref()
  }

  pub fn reset_remote_authentication_client_cert_password(&mut self) {
    self.remote_authentication_client_cert_password = None;
  }

  pub fn set_remote_authentication_scheme(&mut self, remote_authentication_scheme: String) {
    self.remote_authentication_scheme = Some(remote_authentication_scheme);
  }

  pub fn with_remote_authentication_scheme(mut self, remote_authentication_scheme: String) -> MsgVpnBridge {
    self.remote_authentication_scheme = Some(remote_authentication_scheme);
    self
  }

  pub fn remote_authentication_scheme(&self) -> Option<&String> {
    self.remote_authentication_scheme.as_ref()
  }

  pub fn reset_remote_authentication_scheme(&mut self) {
    self.remote_authentication_scheme = None;
  }

  pub fn set_remote_connection_retry_count(&mut self, remote_connection_retry_count: i64) {
    self.remote_connection_retry_count = Some(remote_connection_retry_count);
  }

  pub fn with_remote_connection_retry_count(mut self, remote_connection_retry_count: i64) -> MsgVpnBridge {
    self.remote_connection_retry_count = Some(remote_connection_retry_count);
    self
  }

  pub fn remote_connection_retry_count(&self) -> Option<&i64> {
    self.remote_connection_retry_count.as_ref()
  }

  pub fn reset_remote_connection_retry_count(&mut self) {
    self.remote_connection_retry_count = None;
  }

  pub fn set_remote_connection_retry_delay(&mut self, remote_connection_retry_delay: i64) {
    self.remote_connection_retry_delay = Some(remote_connection_retry_delay);
  }

  pub fn with_remote_connection_retry_delay(mut self, remote_connection_retry_delay: i64) -> MsgVpnBridge {
    self.remote_connection_retry_delay = Some(remote_connection_retry_delay);
    self
  }

  pub fn remote_connection_retry_delay(&self) -> Option<&i64> {
    self.remote_connection_retry_delay.as_ref()
  }

  pub fn reset_remote_connection_retry_delay(&mut self) {
    self.remote_connection_retry_delay = None;
  }

  pub fn set_remote_deliver_to_one_priority(&mut self, remote_deliver_to_one_priority: String) {
    self.remote_deliver_to_one_priority = Some(remote_deliver_to_one_priority);
  }

  pub fn with_remote_deliver_to_one_priority(mut self, remote_deliver_to_one_priority: String) -> MsgVpnBridge {
    self.remote_deliver_to_one_priority = Some(remote_deliver_to_one_priority);
    self
  }

  pub fn remote_deliver_to_one_priority(&self) -> Option<&String> {
    self.remote_deliver_to_one_priority.as_ref()
  }

  pub fn reset_remote_deliver_to_one_priority(&mut self) {
    self.remote_deliver_to_one_priority = None;
  }

  pub fn set_tls_cipher_suite_list(&mut self, tls_cipher_suite_list: String) {
    self.tls_cipher_suite_list = Some(tls_cipher_suite_list);
  }

  pub fn with_tls_cipher_suite_list(mut self, tls_cipher_suite_list: String) -> MsgVpnBridge {
    self.tls_cipher_suite_list = Some(tls_cipher_suite_list);
    self
  }

  pub fn tls_cipher_suite_list(&self) -> Option<&String> {
    self.tls_cipher_suite_list.as_ref()
  }

  pub fn reset_tls_cipher_suite_list(&mut self) {
    self.tls_cipher_suite_list = None;
  }

}



