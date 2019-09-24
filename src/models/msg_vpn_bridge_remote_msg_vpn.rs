/* 
 * SEMP (Solace Element Management Protocol)
 *
 * SEMP (starting in `v2`, see note 1) is a RESTful API for configuring, monitoring, and administering a Solace PubSub+ broker.  SEMP uses URIs to address manageable **resources** of the Solace PubSub+ broker. Resources are individual **objects**, **collections** of objects, or (exclusively in the action API) **actions**. This document applies to the following API:   API|Base Path|Purpose|Comments :---|:---|:---|:--- Configuration|/SEMP/v2/config|Reading and writing config state|See note 2    The following APIs are also available:   API|Base Path|Purpose|Comments :---|:---|:---|:--- Action|/SEMP/v2/action|Performing actions|See note 2 Monitoring|/SEMP/v2/monitor|Querying operational parameters|See note 2    Resources are always nouns, with individual objects being singular and collections being plural.  Objects within a collection are identified by an `obj-id`, which follows the collection name with the form `collection-name/obj-id`.  Actions within an object are identified by an `action-id`, which follows the object name with the form `obj-id/action-id`.  Some examples:  ``` /SEMP/v2/config/msgVpns                        ; MsgVpn collection /SEMP/v2/config/msgVpns/a                      ; MsgVpn object named \"a\" /SEMP/v2/config/msgVpns/a/queues               ; Queue collection in MsgVpn \"a\" /SEMP/v2/config/msgVpns/a/queues/b             ; Queue object named \"b\" in MsgVpn \"a\" /SEMP/v2/action/msgVpns/a/queues/b/startReplay ; Action that starts a replay on Queue \"b\" in MsgVpn \"a\" /SEMP/v2/monitor/msgVpns/a/clients             ; Client collection in MsgVpn \"a\" /SEMP/v2/monitor/msgVpns/a/clients/c           ; Client object named \"c\" in MsgVpn \"a\" ```  ## Collection Resources  Collections are unordered lists of objects (unless described as otherwise), and are described by JSON arrays. Each item in the array represents an object in the same manner as the individual object would normally be represented. In the configuration API, the creation of a new object is done through its collection resource.  ## Object and Action Resources  Objects are composed of attributes, actions, collections, and other objects. They are described by JSON objects as name/value pairs. The collections and actions of an object are not contained directly in the object's JSON content; rather the content includes an attribute containing a URI which points to the collections and actions. These contained resources must be managed through this URI. At a minimum, every object has one or more identifying attributes, and its own `uri` attribute which contains the URI pointing to itself.  Actions are also composed of attributes, and are described by JSON objects as name/value pairs. Unlike objects, however, they are not members of a collection and cannot be retrieved, only performed. Actions only exist in the action API.  Attributes in an object or action may have any (non-exclusively) of the following properties:   Property|Meaning|Comments :---|:---|:--- Identifying|Attribute is involved in unique identification of the object, and appears in its URI| Required|Attribute must be provided in the request| Read-Only|Attribute can only be read, not written|See note 3 Write-Only|Attribute can only be written, not read| Requires-Disable|Attribute can only be changed when object is disabled| Deprecated|Attribute is deprecated, and will disappear in the next SEMP version|    In some requests, certain attributes may only be provided in certain combinations with other attributes:   Relationship|Meaning :---|:--- Requires|Attribute may only be changed by a request if a particular attribute or combination of attributes is also provided in the request Conflicts|Attribute may only be provided in a request if a particular attribute or combination of attributes is not also provided in the request    ## HTTP Methods  The following HTTP methods manipulate resources in accordance with these general principles. Note that some methods are only used in certain APIs:   Method|Resource|Meaning|Request Body|Response Body|Missing Request Attributes :---|:---|:---|:---|:---|:--- POST|Collection|Create object|Initial attribute values|Object attributes and metadata|Set to default PUT|Object|Create or replace object|New attribute values|Object attributes and metadata|Set to default (but see note 4) PUT|Action|Performs action|Action arguments|Action metadata|N/A PATCH|Object|Update object|New attribute values|Object attributes and metadata|unchanged DELETE|Object|Delete object|Empty|Object metadata|N/A GET|Object|Get object|Empty|Object attributes and metadata|N/A GET|Collection|Get collection|Empty|Object attributes and collection metadata|N/A    ## Common Query Parameters  The following are some common query parameters that are supported by many method/URI combinations. Individual URIs may document additional parameters. Note that multiple query parameters can be used together in a single URI, separated by the ampersand character. For example:  ``` ; Request for the MsgVpns collection using two hypothetical query parameters \"q1\" and \"q2\" ; with values \"val1\" and \"val2\" respectively /SEMP/v2/config/msgVpns?q1=val1&q2=val2 ```  ### select  Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. Use this query parameter to limit the size of the returned data for each returned object, return only those fields that are desired, or exclude fields that are not desired.  The value of `select` is a comma-separated list of attribute names. If the list contains attribute names that are not prefaced by `-`, only those attributes are included in the response. If the list contains attribute names that are prefaced by `-`, those attributes are excluded from the response. If the list contains both types, then the difference of the first set of attributes and the second set of attributes is returned. If the list is empty (i.e. `select=`), no attributes are returned.  All attributes that are prefaced by `-` must follow all attributes that are not prefaced by `-`. In addition, each attribute name in the list must match at least one attribute in the object.  Names may include the `*` wildcard (zero or more characters). Nested attribute names are supported using periods (e.g. `parentName.childName`).  Some examples:  ``` ; List of all MsgVpn names /SEMP/v2/config/msgVpns?select=msgVpnName ; List of all MsgVpn and their attributes except for their names /SEMP/v2/config/msgVpns?select=-msgVpnName ; Authentication attributes of MsgVpn \"finance\" /SEMP/v2/config/msgVpns/finance?select=authentication* ; All attributes of MsgVpn \"finance\" except for authentication attributes /SEMP/v2/config/msgVpns/finance?select=-authentication* ; Access related attributes of Queue \"orderQ\" of MsgVpn \"finance\" /SEMP/v2/config/msgVpns/finance/queues/orderQ?select=owner,permission ```  ### where  Include in the response only objects where certain conditions are true. Use this query parameter to limit which objects are returned to those whose attribute values meet the given conditions.  The value of `where` is a comma-separated list of expressions. All expressions must be true for the object to be included in the response. Each expression takes the form:  ``` expression  = attribute-name OP value OP          = '==' | '!=' | '&lt;' | '&gt;' | '&lt;=' | '&gt;=' ```  `value` may be a number, string, `true`, or `false`, as appropriate for the type of `attribute-name`. Greater-than and less-than comparisons only work for numbers. A `*` in a string `value` is interpreted as a wildcard (zero or more characters). Some examples:  ``` ; Only enabled MsgVpns /SEMP/v2/config/msgVpns?where=enabled==true ; Only MsgVpns using basic non-LDAP authentication /SEMP/v2/config/msgVpns?where=authenticationBasicEnabled==true,authenticationBasicType!=ldap ; Only MsgVpns that allow more than 100 client connections /SEMP/v2/config/msgVpns?where=maxConnectionCount>100 ; Only MsgVpns with msgVpnName starting with \"B\": /SEMP/v2/config/msgVpns?where=msgVpnName==B* ```  ### count  Limit the count of objects in the response. This can be useful to limit the size of the response for large collections. The minimum value for `count` is `1` and the default is `10`. There is also a per-collection maximum value to limit request handling time. For example:  ``` ; Up to 25 MsgVpns /SEMP/v2/config/msgVpns?count=25 ```  ### cursor  The cursor, or position, for the next page of objects. Cursors are opaque data that should not be created or interpreted by SEMP clients, and should only be used as described below.  When a request is made for a collection and there may be additional objects available for retrieval that are not included in the initial response, the response will include a `cursorQuery` field containing a cursor. The value of this field can be specified in the `cursor` query parameter of a subsequent request to retrieve the next page of objects. For convenience, an appropriate URI is constructed automatically by the broker and included in the `nextPageUri` field of the response. This URI can be used directly to retrieve the next page of objects.  ## Notes  Note|Description :---:|:--- 1|This specification defines SEMP starting in \"v2\", and not the original SEMP \"v1\" interface. Request and response formats between \"v1\" and \"v2\" are entirely incompatible, although both protocols share a common port configuration on the Solace PubSub+ broker. They are differentiated by the initial portion of the URI path, one of either \"/SEMP/\" or \"/SEMP/v2/\" 2|This API is partially implemented. Only a subset of all objects are available. 3|Read-only attributes may appear in POST and PUT/PATCH requests. However, if a read-only attribute is not marked as identifying, it will be ignored during a PUT/PATCH. 4|For PUT, if the SEMP user is not authorized to modify the attribute, its value is left unchanged rather than set to default. In addition, the values of write-only attributes are not set to their defaults on a PUT. If the object does not exist, it is created first.    
 *
 * OpenAPI spec version: 2.12.00902000014
 * Contact: support@solace.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MsgVpnBridgeRemoteMsgVpn {
  /// The name of the Bridge.
  #[serde(rename = "bridgeName", skip_serializing_if="Option::is_none")]
  bridge_name: Option<String>,
  /// The virtual router of the Bridge. The allowed values and their meaning are:  <pre> \"primary\" - The Bridge is used for the primary virtual router. \"backup\" - The Bridge is used for the backup virtual router. \"auto\" - The Bridge is automatically assigned a router. </pre> 
  #[serde(rename = "bridgeVirtualRouter", skip_serializing_if="Option::is_none")]
  bridge_virtual_router: Option<String>,
  /// The Client Username the Bridge uses to login to the remote Message VPN. This per remote Message VPN value overrides the value provided for the Bridge overall. The default value is `\"\"`.
  #[serde(rename = "clientUsername", skip_serializing_if="Option::is_none")]
  client_username: Option<String>,
  /// Enable or disable data compression for the remote Message VPN connection. The default value is `false`.
  #[serde(rename = "compressedDataEnabled", skip_serializing_if="Option::is_none")]
  compressed_data_enabled: Option<bool>,
  /// The preference given to incoming connections from remote Message VPN hosts, from 1 (highest priority) to 4 (lowest priority). The default value is `4`.
  #[serde(rename = "connectOrder", skip_serializing_if="Option::is_none")]
  connect_order: Option<i32>,
  /// The number of outstanding guaranteed messages that can be transmitted over the remote Message VPN connection before an acknowledgement is received. The default value is `255`.
  #[serde(rename = "egressFlowWindowSize", skip_serializing_if="Option::is_none")]
  egress_flow_window_size: Option<i64>,
  /// Enable or disable the remote Message VPN. The default value is `false`.
  #[serde(rename = "enabled", skip_serializing_if="Option::is_none")]
  enabled: Option<bool>,
  /// The name of the Message VPN.
  #[serde(rename = "msgVpnName", skip_serializing_if="Option::is_none")]
  msg_vpn_name: Option<String>,
  /// The password for the Client Username. The default is to have no `password`.
  #[serde(rename = "password", skip_serializing_if="Option::is_none")]
  password: Option<String>,
  /// The queue binding of the Bridge in the remote Message VPN. The default value is `\"\"`.
  #[serde(rename = "queueBinding", skip_serializing_if="Option::is_none")]
  queue_binding: Option<String>,
  /// The physical interface on the local Message VPN host for connecting to the remote Message VPN. By default, an interface is chosen automatically (recommended), but if specified, `remoteMsgVpnLocation` must not be a virtual router name.
  #[serde(rename = "remoteMsgVpnInterface", skip_serializing_if="Option::is_none")]
  remote_msg_vpn_interface: Option<String>,
  /// The location of the remote Message VPN as either an FQDN with port, IP address with port, or virtual router name (starting with \"v:\").
  #[serde(rename = "remoteMsgVpnLocation", skip_serializing_if="Option::is_none")]
  remote_msg_vpn_location: Option<String>,
  /// The name of the remote Message VPN.
  #[serde(rename = "remoteMsgVpnName", skip_serializing_if="Option::is_none")]
  remote_msg_vpn_name: Option<String>,
  /// Enable or disable TLS encryption for the remote Message VPN connection. The default value is `false`.
  #[serde(rename = "tlsEnabled", skip_serializing_if="Option::is_none")]
  tls_enabled: Option<bool>,
  /// The Client Profile for the unidirectional Bridge of the remote Message VPN. The Client Profile must exist in the local Message VPN, and it is used only for the TCP parameters. The default value is `\"#client-profile\"`.
  #[serde(rename = "unidirectionalClientProfile", skip_serializing_if="Option::is_none")]
  unidirectional_client_profile: Option<String>
}

impl MsgVpnBridgeRemoteMsgVpn {
  pub fn new() -> MsgVpnBridgeRemoteMsgVpn {
    MsgVpnBridgeRemoteMsgVpn {
      bridge_name: None,
      bridge_virtual_router: None,
      client_username: None,
      compressed_data_enabled: None,
      connect_order: None,
      egress_flow_window_size: None,
      enabled: None,
      msg_vpn_name: None,
      password: None,
      queue_binding: None,
      remote_msg_vpn_interface: None,
      remote_msg_vpn_location: None,
      remote_msg_vpn_name: None,
      tls_enabled: None,
      unidirectional_client_profile: None
    }
  }

  pub fn set_bridge_name(&mut self, bridge_name: String) {
    self.bridge_name = Some(bridge_name);
  }

  pub fn with_bridge_name(mut self, bridge_name: String) -> MsgVpnBridgeRemoteMsgVpn {
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

  pub fn with_bridge_virtual_router(mut self, bridge_virtual_router: String) -> MsgVpnBridgeRemoteMsgVpn {
    self.bridge_virtual_router = Some(bridge_virtual_router);
    self
  }

  pub fn bridge_virtual_router(&self) -> Option<&String> {
    self.bridge_virtual_router.as_ref()
  }

  pub fn reset_bridge_virtual_router(&mut self) {
    self.bridge_virtual_router = None;
  }

  pub fn set_client_username(&mut self, client_username: String) {
    self.client_username = Some(client_username);
  }

  pub fn with_client_username(mut self, client_username: String) -> MsgVpnBridgeRemoteMsgVpn {
    self.client_username = Some(client_username);
    self
  }

  pub fn client_username(&self) -> Option<&String> {
    self.client_username.as_ref()
  }

  pub fn reset_client_username(&mut self) {
    self.client_username = None;
  }

  pub fn set_compressed_data_enabled(&mut self, compressed_data_enabled: bool) {
    self.compressed_data_enabled = Some(compressed_data_enabled);
  }

  pub fn with_compressed_data_enabled(mut self, compressed_data_enabled: bool) -> MsgVpnBridgeRemoteMsgVpn {
    self.compressed_data_enabled = Some(compressed_data_enabled);
    self
  }

  pub fn compressed_data_enabled(&self) -> Option<&bool> {
    self.compressed_data_enabled.as_ref()
  }

  pub fn reset_compressed_data_enabled(&mut self) {
    self.compressed_data_enabled = None;
  }

  pub fn set_connect_order(&mut self, connect_order: i32) {
    self.connect_order = Some(connect_order);
  }

  pub fn with_connect_order(mut self, connect_order: i32) -> MsgVpnBridgeRemoteMsgVpn {
    self.connect_order = Some(connect_order);
    self
  }

  pub fn connect_order(&self) -> Option<&i32> {
    self.connect_order.as_ref()
  }

  pub fn reset_connect_order(&mut self) {
    self.connect_order = None;
  }

  pub fn set_egress_flow_window_size(&mut self, egress_flow_window_size: i64) {
    self.egress_flow_window_size = Some(egress_flow_window_size);
  }

  pub fn with_egress_flow_window_size(mut self, egress_flow_window_size: i64) -> MsgVpnBridgeRemoteMsgVpn {
    self.egress_flow_window_size = Some(egress_flow_window_size);
    self
  }

  pub fn egress_flow_window_size(&self) -> Option<&i64> {
    self.egress_flow_window_size.as_ref()
  }

  pub fn reset_egress_flow_window_size(&mut self) {
    self.egress_flow_window_size = None;
  }

  pub fn set_enabled(&mut self, enabled: bool) {
    self.enabled = Some(enabled);
  }

  pub fn with_enabled(mut self, enabled: bool) -> MsgVpnBridgeRemoteMsgVpn {
    self.enabled = Some(enabled);
    self
  }

  pub fn enabled(&self) -> Option<&bool> {
    self.enabled.as_ref()
  }

  pub fn reset_enabled(&mut self) {
    self.enabled = None;
  }

  pub fn set_msg_vpn_name(&mut self, msg_vpn_name: String) {
    self.msg_vpn_name = Some(msg_vpn_name);
  }

  pub fn with_msg_vpn_name(mut self, msg_vpn_name: String) -> MsgVpnBridgeRemoteMsgVpn {
    self.msg_vpn_name = Some(msg_vpn_name);
    self
  }

  pub fn msg_vpn_name(&self) -> Option<&String> {
    self.msg_vpn_name.as_ref()
  }

  pub fn reset_msg_vpn_name(&mut self) {
    self.msg_vpn_name = None;
  }

  pub fn set_password(&mut self, password: String) {
    self.password = Some(password);
  }

  pub fn with_password(mut self, password: String) -> MsgVpnBridgeRemoteMsgVpn {
    self.password = Some(password);
    self
  }

  pub fn password(&self) -> Option<&String> {
    self.password.as_ref()
  }

  pub fn reset_password(&mut self) {
    self.password = None;
  }

  pub fn set_queue_binding(&mut self, queue_binding: String) {
    self.queue_binding = Some(queue_binding);
  }

  pub fn with_queue_binding(mut self, queue_binding: String) -> MsgVpnBridgeRemoteMsgVpn {
    self.queue_binding = Some(queue_binding);
    self
  }

  pub fn queue_binding(&self) -> Option<&String> {
    self.queue_binding.as_ref()
  }

  pub fn reset_queue_binding(&mut self) {
    self.queue_binding = None;
  }

  pub fn set_remote_msg_vpn_interface(&mut self, remote_msg_vpn_interface: String) {
    self.remote_msg_vpn_interface = Some(remote_msg_vpn_interface);
  }

  pub fn with_remote_msg_vpn_interface(mut self, remote_msg_vpn_interface: String) -> MsgVpnBridgeRemoteMsgVpn {
    self.remote_msg_vpn_interface = Some(remote_msg_vpn_interface);
    self
  }

  pub fn remote_msg_vpn_interface(&self) -> Option<&String> {
    self.remote_msg_vpn_interface.as_ref()
  }

  pub fn reset_remote_msg_vpn_interface(&mut self) {
    self.remote_msg_vpn_interface = None;
  }

  pub fn set_remote_msg_vpn_location(&mut self, remote_msg_vpn_location: String) {
    self.remote_msg_vpn_location = Some(remote_msg_vpn_location);
  }

  pub fn with_remote_msg_vpn_location(mut self, remote_msg_vpn_location: String) -> MsgVpnBridgeRemoteMsgVpn {
    self.remote_msg_vpn_location = Some(remote_msg_vpn_location);
    self
  }

  pub fn remote_msg_vpn_location(&self) -> Option<&String> {
    self.remote_msg_vpn_location.as_ref()
  }

  pub fn reset_remote_msg_vpn_location(&mut self) {
    self.remote_msg_vpn_location = None;
  }

  pub fn set_remote_msg_vpn_name(&mut self, remote_msg_vpn_name: String) {
    self.remote_msg_vpn_name = Some(remote_msg_vpn_name);
  }

  pub fn with_remote_msg_vpn_name(mut self, remote_msg_vpn_name: String) -> MsgVpnBridgeRemoteMsgVpn {
    self.remote_msg_vpn_name = Some(remote_msg_vpn_name);
    self
  }

  pub fn remote_msg_vpn_name(&self) -> Option<&String> {
    self.remote_msg_vpn_name.as_ref()
  }

  pub fn reset_remote_msg_vpn_name(&mut self) {
    self.remote_msg_vpn_name = None;
  }

  pub fn set_tls_enabled(&mut self, tls_enabled: bool) {
    self.tls_enabled = Some(tls_enabled);
  }

  pub fn with_tls_enabled(mut self, tls_enabled: bool) -> MsgVpnBridgeRemoteMsgVpn {
    self.tls_enabled = Some(tls_enabled);
    self
  }

  pub fn tls_enabled(&self) -> Option<&bool> {
    self.tls_enabled.as_ref()
  }

  pub fn reset_tls_enabled(&mut self) {
    self.tls_enabled = None;
  }

  pub fn set_unidirectional_client_profile(&mut self, unidirectional_client_profile: String) {
    self.unidirectional_client_profile = Some(unidirectional_client_profile);
  }

  pub fn with_unidirectional_client_profile(mut self, unidirectional_client_profile: String) -> MsgVpnBridgeRemoteMsgVpn {
    self.unidirectional_client_profile = Some(unidirectional_client_profile);
    self
  }

  pub fn unidirectional_client_profile(&self) -> Option<&String> {
    self.unidirectional_client_profile.as_ref()
  }

  pub fn reset_unidirectional_client_profile(&mut self) {
    self.unidirectional_client_profile = None;
  }

}



