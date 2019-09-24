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
pub struct DmrClusterLink {
  /// The password used to authenticate with the remote node when using basic internal authentication. If this per-Link password is not configured, the Cluster's password is used instead. The default is to have no `authenticationBasicPassword`.
  #[serde(rename = "authenticationBasicPassword", skip_serializing_if="Option::is_none")]
  authentication_basic_password: Option<String>,
  /// The authentication scheme to be used by the Link which initiates connections to the remote node. The default value is `\"basic\"`. The allowed values and their meaning are:  <pre> \"basic\" - Basic Authentication Scheme (via username and password). \"client-certificate\" - Client Certificate Authentication Scheme (via certificate file or content). </pre> 
  #[serde(rename = "authenticationScheme", skip_serializing_if="Option::is_none")]
  authentication_scheme: Option<String>,
  /// The maximum depth of the \"Control 1\" (C-1) priority queue, in work units. Each work unit is 2048 bytes of message data. The default value is `20000`.
  #[serde(rename = "clientProfileQueueControl1MaxDepth", skip_serializing_if="Option::is_none")]
  client_profile_queue_control1_max_depth: Option<i32>,
  /// The number of messages that are always allowed entry into the \"Control 1\" (C-1) priority queue, regardless of the `clientProfileQueueControl1MaxDepth` value. The default value is `4`.
  #[serde(rename = "clientProfileQueueControl1MinMsgBurst", skip_serializing_if="Option::is_none")]
  client_profile_queue_control1_min_msg_burst: Option<i32>,
  /// The maximum depth of the \"Direct 1\" (D-1) priority queue, in work units. Each work unit is 2048 bytes of message data. The default value is `20000`.
  #[serde(rename = "clientProfileQueueDirect1MaxDepth", skip_serializing_if="Option::is_none")]
  client_profile_queue_direct1_max_depth: Option<i32>,
  /// The number of messages that are always allowed entry into the \"Direct 1\" (D-1) priority queue, regardless of the `clientProfileQueueDirect1MaxDepth` value. The default value is `4`.
  #[serde(rename = "clientProfileQueueDirect1MinMsgBurst", skip_serializing_if="Option::is_none")]
  client_profile_queue_direct1_min_msg_burst: Option<i32>,
  /// The maximum depth of the \"Direct 2\" (D-2) priority queue, in work units. Each work unit is 2048 bytes of message data. The default value is `20000`.
  #[serde(rename = "clientProfileQueueDirect2MaxDepth", skip_serializing_if="Option::is_none")]
  client_profile_queue_direct2_max_depth: Option<i32>,
  /// The number of messages that are always allowed entry into the \"Direct 2\" (D-2) priority queue, regardless of the `clientProfileQueueDirect2MaxDepth` value. The default value is `4`.
  #[serde(rename = "clientProfileQueueDirect2MinMsgBurst", skip_serializing_if="Option::is_none")]
  client_profile_queue_direct2_min_msg_burst: Option<i32>,
  /// The maximum depth of the \"Direct 3\" (D-3) priority queue, in work units. Each work unit is 2048 bytes of message data. The default value is `20000`.
  #[serde(rename = "clientProfileQueueDirect3MaxDepth", skip_serializing_if="Option::is_none")]
  client_profile_queue_direct3_max_depth: Option<i32>,
  /// The number of messages that are always allowed entry into the \"Direct 3\" (D-3) priority queue, regardless of the `clientProfileQueueDirect3MaxDepth` value. The default value is `4`.
  #[serde(rename = "clientProfileQueueDirect3MinMsgBurst", skip_serializing_if="Option::is_none")]
  client_profile_queue_direct3_min_msg_burst: Option<i32>,
  /// The maximum depth of the \"Guaranteed 1\" (G-1) priority queue, in work units. Each work unit is 2048 bytes of message data. The default value is `20000`.
  #[serde(rename = "clientProfileQueueGuaranteed1MaxDepth", skip_serializing_if="Option::is_none")]
  client_profile_queue_guaranteed1_max_depth: Option<i32>,
  /// The number of messages that are always allowed entry into the \"Guaranteed 1\" (G-3) priority queue, regardless of the `clientProfileQueueGuaranteed1MaxDepth` value. The default value is `255`.
  #[serde(rename = "clientProfileQueueGuaranteed1MinMsgBurst", skip_serializing_if="Option::is_none")]
  client_profile_queue_guaranteed1_min_msg_burst: Option<i32>,
  /// The TCP initial congestion window size, in multiples of the TCP Maximum Segment Size (MSS). Changing the value from its default of 2 results in non-compliance with RFC 2581. Contact Solace Support before changing this value. The default value is `2`.
  #[serde(rename = "clientProfileTcpCongestionWindowSize", skip_serializing_if="Option::is_none")]
  client_profile_tcp_congestion_window_size: Option<i64>,
  /// The number of TCP keepalive retransmissions to be carried out before declaring that the remote end is not available. The default value is `5`.
  #[serde(rename = "clientProfileTcpKeepaliveCount", skip_serializing_if="Option::is_none")]
  client_profile_tcp_keepalive_count: Option<i64>,
  /// The amount of time a connection must remain idle before TCP begins sending keepalive probes, in seconds. The default value is `3`.
  #[serde(rename = "clientProfileTcpKeepaliveIdleTime", skip_serializing_if="Option::is_none")]
  client_profile_tcp_keepalive_idle_time: Option<i64>,
  /// The amount of time between TCP keepalive retransmissions when no acknowledgement is received, in seconds. The default value is `1`.
  #[serde(rename = "clientProfileTcpKeepaliveInterval", skip_serializing_if="Option::is_none")]
  client_profile_tcp_keepalive_interval: Option<i64>,
  /// The TCP maximum segment size, in kilobytes. Changes are applied to all existing connections. The default value is `1460`.
  #[serde(rename = "clientProfileTcpMaxSegmentSize", skip_serializing_if="Option::is_none")]
  client_profile_tcp_max_segment_size: Option<i64>,
  /// The TCP maximum window size, in kilobytes. Changes are applied to all existing connections. The default value is `256`.
  #[serde(rename = "clientProfileTcpMaxWindowSize", skip_serializing_if="Option::is_none")]
  client_profile_tcp_max_window_size: Option<i64>,
  /// The name of the Cluster.
  #[serde(rename = "dmrClusterName", skip_serializing_if="Option::is_none")]
  dmr_cluster_name: Option<String>,
  /// The number of outstanding guaranteed messages that can be sent over the Link before acknowledgement is received by the sender. The default value is `255`.
  #[serde(rename = "egressFlowWindowSize", skip_serializing_if="Option::is_none")]
  egress_flow_window_size: Option<i64>,
  /// Enable or disable the Link. When disabled, subscription sets of this and the remote node are not kept up-to-date, and messages are not exchanged with the remote node. Published guaranteed messages will be queued up for future delivery based on current subscription sets. The default value is `false`.
  #[serde(rename = "enabled", skip_serializing_if="Option::is_none")]
  enabled: Option<bool>,
  /// The initiator of the Link's TCP connections. The default value is `\"lexical\"`. The allowed values and their meaning are:  <pre> \"lexical\" - The \"higher\" node-name initiates. \"local\" - The local node initiates. \"remote\" - The remote node initiates. </pre> 
  #[serde(rename = "initiator", skip_serializing_if="Option::is_none")]
  initiator: Option<String>,
  /// The name of the Dead Message Queue (DMQ) used by the Queue for discarded messages. The default value is `\"#DEAD_MSG_QUEUE\"`.
  #[serde(rename = "queueDeadMsgQueue", skip_serializing_if="Option::is_none")]
  queue_dead_msg_queue: Option<String>,
  #[serde(rename = "queueEventSpoolUsageThreshold", skip_serializing_if="Option::is_none")]
  queue_event_spool_usage_threshold: Option<::models::EventThreshold>,
  /// The maximum number of messages delivered but not acknowledged per flow for the Queue. The default is the max value supported by the platform.
  #[serde(rename = "queueMaxDeliveredUnackedMsgsPerFlow", skip_serializing_if="Option::is_none")]
  queue_max_delivered_unacked_msgs_per_flow: Option<i64>,
  /// The maximum message spool usage by the Queue (quota), in megabytes (MB). The default varies by platform.
  #[serde(rename = "queueMaxMsgSpoolUsage", skip_serializing_if="Option::is_none")]
  queue_max_msg_spool_usage: Option<i64>,
  /// The maximum number of times the Queue will attempt redelivery of a message prior to it being discarded or moved to the DMQ. A value of 0 means to retry forever. The default value is `0`.
  #[serde(rename = "queueMaxRedeliveryCount", skip_serializing_if="Option::is_none")]
  queue_max_redelivery_count: Option<i64>,
  /// The maximum time in seconds a message can stay in the Queue when `queueRespectTtlEnabled` is `true`. A message expires when the lesser of the sender assigned time-to-live (TTL) in the message and the `queueMaxTtl` configured for the Queue, is exceeded. A value of 0 disables expiry. The default value is `0`.
  #[serde(rename = "queueMaxTtl", skip_serializing_if="Option::is_none")]
  queue_max_ttl: Option<i64>,
  /// Determines when to return negative acknowledgements (NACKs) to sending clients on message discards. Note that NACKs cause the message to not be delivered to any destination and Transacted Session commits to fail. The default value is `\"always\"`. The allowed values and their meaning are:  <pre> \"always\" - Always return a negative acknowledgment (NACK) to the sending client on message discard. \"when-queue-enabled\" - Only return a negative acknowledgment (NACK) to the sending client on message discard when the Queue is enabled. \"never\" - Never return a negative acknowledgment (NACK) to the sending client on message discard. </pre> 
  #[serde(rename = "queueRejectMsgToSenderOnDiscardBehavior", skip_serializing_if="Option::is_none")]
  queue_reject_msg_to_sender_on_discard_behavior: Option<String>,
  /// Enable or disable the respecting of the time-to-live (TTL) for messages in the Queue. When enabled, expired messages are discarded or moved to the DMQ. The default value is `false`.
  #[serde(rename = "queueRespectTtlEnabled", skip_serializing_if="Option::is_none")]
  queue_respect_ttl_enabled: Option<bool>,
  /// The name of the node at the remote end of the Link.
  #[serde(rename = "remoteNodeName", skip_serializing_if="Option::is_none")]
  remote_node_name: Option<String>,
  /// The span of the Link, either internal or external. Internal Links connect nodes within the same Cluster. External Links connect nodes within different Clusters. The default value is `\"external\"`. The allowed values and their meaning are:  <pre> \"internal\" - Link to same cluster. \"external\" - Link to other cluster. </pre> 
  #[serde(rename = "span", skip_serializing_if="Option::is_none")]
  span: Option<String>,
  /// Enable or disable compression on the Link. The default value is `false`.
  #[serde(rename = "transportCompressedEnabled", skip_serializing_if="Option::is_none")]
  transport_compressed_enabled: Option<bool>,
  /// Enable or disable encryption on the Link. The default value is `false`.
  #[serde(rename = "transportTlsEnabled", skip_serializing_if="Option::is_none")]
  transport_tls_enabled: Option<bool>
}

impl DmrClusterLink {
  pub fn new() -> DmrClusterLink {
    DmrClusterLink {
      authentication_basic_password: None,
      authentication_scheme: None,
      client_profile_queue_control1_max_depth: None,
      client_profile_queue_control1_min_msg_burst: None,
      client_profile_queue_direct1_max_depth: None,
      client_profile_queue_direct1_min_msg_burst: None,
      client_profile_queue_direct2_max_depth: None,
      client_profile_queue_direct2_min_msg_burst: None,
      client_profile_queue_direct3_max_depth: None,
      client_profile_queue_direct3_min_msg_burst: None,
      client_profile_queue_guaranteed1_max_depth: None,
      client_profile_queue_guaranteed1_min_msg_burst: None,
      client_profile_tcp_congestion_window_size: None,
      client_profile_tcp_keepalive_count: None,
      client_profile_tcp_keepalive_idle_time: None,
      client_profile_tcp_keepalive_interval: None,
      client_profile_tcp_max_segment_size: None,
      client_profile_tcp_max_window_size: None,
      dmr_cluster_name: None,
      egress_flow_window_size: None,
      enabled: None,
      initiator: None,
      queue_dead_msg_queue: None,
      queue_event_spool_usage_threshold: None,
      queue_max_delivered_unacked_msgs_per_flow: None,
      queue_max_msg_spool_usage: None,
      queue_max_redelivery_count: None,
      queue_max_ttl: None,
      queue_reject_msg_to_sender_on_discard_behavior: None,
      queue_respect_ttl_enabled: None,
      remote_node_name: None,
      span: None,
      transport_compressed_enabled: None,
      transport_tls_enabled: None
    }
  }

  pub fn set_authentication_basic_password(&mut self, authentication_basic_password: String) {
    self.authentication_basic_password = Some(authentication_basic_password);
  }

  pub fn with_authentication_basic_password(mut self, authentication_basic_password: String) -> DmrClusterLink {
    self.authentication_basic_password = Some(authentication_basic_password);
    self
  }

  pub fn authentication_basic_password(&self) -> Option<&String> {
    self.authentication_basic_password.as_ref()
  }

  pub fn reset_authentication_basic_password(&mut self) {
    self.authentication_basic_password = None;
  }

  pub fn set_authentication_scheme(&mut self, authentication_scheme: String) {
    self.authentication_scheme = Some(authentication_scheme);
  }

  pub fn with_authentication_scheme(mut self, authentication_scheme: String) -> DmrClusterLink {
    self.authentication_scheme = Some(authentication_scheme);
    self
  }

  pub fn authentication_scheme(&self) -> Option<&String> {
    self.authentication_scheme.as_ref()
  }

  pub fn reset_authentication_scheme(&mut self) {
    self.authentication_scheme = None;
  }

  pub fn set_client_profile_queue_control1_max_depth(&mut self, client_profile_queue_control1_max_depth: i32) {
    self.client_profile_queue_control1_max_depth = Some(client_profile_queue_control1_max_depth);
  }

  pub fn with_client_profile_queue_control1_max_depth(mut self, client_profile_queue_control1_max_depth: i32) -> DmrClusterLink {
    self.client_profile_queue_control1_max_depth = Some(client_profile_queue_control1_max_depth);
    self
  }

  pub fn client_profile_queue_control1_max_depth(&self) -> Option<&i32> {
    self.client_profile_queue_control1_max_depth.as_ref()
  }

  pub fn reset_client_profile_queue_control1_max_depth(&mut self) {
    self.client_profile_queue_control1_max_depth = None;
  }

  pub fn set_client_profile_queue_control1_min_msg_burst(&mut self, client_profile_queue_control1_min_msg_burst: i32) {
    self.client_profile_queue_control1_min_msg_burst = Some(client_profile_queue_control1_min_msg_burst);
  }

  pub fn with_client_profile_queue_control1_min_msg_burst(mut self, client_profile_queue_control1_min_msg_burst: i32) -> DmrClusterLink {
    self.client_profile_queue_control1_min_msg_burst = Some(client_profile_queue_control1_min_msg_burst);
    self
  }

  pub fn client_profile_queue_control1_min_msg_burst(&self) -> Option<&i32> {
    self.client_profile_queue_control1_min_msg_burst.as_ref()
  }

  pub fn reset_client_profile_queue_control1_min_msg_burst(&mut self) {
    self.client_profile_queue_control1_min_msg_burst = None;
  }

  pub fn set_client_profile_queue_direct1_max_depth(&mut self, client_profile_queue_direct1_max_depth: i32) {
    self.client_profile_queue_direct1_max_depth = Some(client_profile_queue_direct1_max_depth);
  }

  pub fn with_client_profile_queue_direct1_max_depth(mut self, client_profile_queue_direct1_max_depth: i32) -> DmrClusterLink {
    self.client_profile_queue_direct1_max_depth = Some(client_profile_queue_direct1_max_depth);
    self
  }

  pub fn client_profile_queue_direct1_max_depth(&self) -> Option<&i32> {
    self.client_profile_queue_direct1_max_depth.as_ref()
  }

  pub fn reset_client_profile_queue_direct1_max_depth(&mut self) {
    self.client_profile_queue_direct1_max_depth = None;
  }

  pub fn set_client_profile_queue_direct1_min_msg_burst(&mut self, client_profile_queue_direct1_min_msg_burst: i32) {
    self.client_profile_queue_direct1_min_msg_burst = Some(client_profile_queue_direct1_min_msg_burst);
  }

  pub fn with_client_profile_queue_direct1_min_msg_burst(mut self, client_profile_queue_direct1_min_msg_burst: i32) -> DmrClusterLink {
    self.client_profile_queue_direct1_min_msg_burst = Some(client_profile_queue_direct1_min_msg_burst);
    self
  }

  pub fn client_profile_queue_direct1_min_msg_burst(&self) -> Option<&i32> {
    self.client_profile_queue_direct1_min_msg_burst.as_ref()
  }

  pub fn reset_client_profile_queue_direct1_min_msg_burst(&mut self) {
    self.client_profile_queue_direct1_min_msg_burst = None;
  }

  pub fn set_client_profile_queue_direct2_max_depth(&mut self, client_profile_queue_direct2_max_depth: i32) {
    self.client_profile_queue_direct2_max_depth = Some(client_profile_queue_direct2_max_depth);
  }

  pub fn with_client_profile_queue_direct2_max_depth(mut self, client_profile_queue_direct2_max_depth: i32) -> DmrClusterLink {
    self.client_profile_queue_direct2_max_depth = Some(client_profile_queue_direct2_max_depth);
    self
  }

  pub fn client_profile_queue_direct2_max_depth(&self) -> Option<&i32> {
    self.client_profile_queue_direct2_max_depth.as_ref()
  }

  pub fn reset_client_profile_queue_direct2_max_depth(&mut self) {
    self.client_profile_queue_direct2_max_depth = None;
  }

  pub fn set_client_profile_queue_direct2_min_msg_burst(&mut self, client_profile_queue_direct2_min_msg_burst: i32) {
    self.client_profile_queue_direct2_min_msg_burst = Some(client_profile_queue_direct2_min_msg_burst);
  }

  pub fn with_client_profile_queue_direct2_min_msg_burst(mut self, client_profile_queue_direct2_min_msg_burst: i32) -> DmrClusterLink {
    self.client_profile_queue_direct2_min_msg_burst = Some(client_profile_queue_direct2_min_msg_burst);
    self
  }

  pub fn client_profile_queue_direct2_min_msg_burst(&self) -> Option<&i32> {
    self.client_profile_queue_direct2_min_msg_burst.as_ref()
  }

  pub fn reset_client_profile_queue_direct2_min_msg_burst(&mut self) {
    self.client_profile_queue_direct2_min_msg_burst = None;
  }

  pub fn set_client_profile_queue_direct3_max_depth(&mut self, client_profile_queue_direct3_max_depth: i32) {
    self.client_profile_queue_direct3_max_depth = Some(client_profile_queue_direct3_max_depth);
  }

  pub fn with_client_profile_queue_direct3_max_depth(mut self, client_profile_queue_direct3_max_depth: i32) -> DmrClusterLink {
    self.client_profile_queue_direct3_max_depth = Some(client_profile_queue_direct3_max_depth);
    self
  }

  pub fn client_profile_queue_direct3_max_depth(&self) -> Option<&i32> {
    self.client_profile_queue_direct3_max_depth.as_ref()
  }

  pub fn reset_client_profile_queue_direct3_max_depth(&mut self) {
    self.client_profile_queue_direct3_max_depth = None;
  }

  pub fn set_client_profile_queue_direct3_min_msg_burst(&mut self, client_profile_queue_direct3_min_msg_burst: i32) {
    self.client_profile_queue_direct3_min_msg_burst = Some(client_profile_queue_direct3_min_msg_burst);
  }

  pub fn with_client_profile_queue_direct3_min_msg_burst(mut self, client_profile_queue_direct3_min_msg_burst: i32) -> DmrClusterLink {
    self.client_profile_queue_direct3_min_msg_burst = Some(client_profile_queue_direct3_min_msg_burst);
    self
  }

  pub fn client_profile_queue_direct3_min_msg_burst(&self) -> Option<&i32> {
    self.client_profile_queue_direct3_min_msg_burst.as_ref()
  }

  pub fn reset_client_profile_queue_direct3_min_msg_burst(&mut self) {
    self.client_profile_queue_direct3_min_msg_burst = None;
  }

  pub fn set_client_profile_queue_guaranteed1_max_depth(&mut self, client_profile_queue_guaranteed1_max_depth: i32) {
    self.client_profile_queue_guaranteed1_max_depth = Some(client_profile_queue_guaranteed1_max_depth);
  }

  pub fn with_client_profile_queue_guaranteed1_max_depth(mut self, client_profile_queue_guaranteed1_max_depth: i32) -> DmrClusterLink {
    self.client_profile_queue_guaranteed1_max_depth = Some(client_profile_queue_guaranteed1_max_depth);
    self
  }

  pub fn client_profile_queue_guaranteed1_max_depth(&self) -> Option<&i32> {
    self.client_profile_queue_guaranteed1_max_depth.as_ref()
  }

  pub fn reset_client_profile_queue_guaranteed1_max_depth(&mut self) {
    self.client_profile_queue_guaranteed1_max_depth = None;
  }

  pub fn set_client_profile_queue_guaranteed1_min_msg_burst(&mut self, client_profile_queue_guaranteed1_min_msg_burst: i32) {
    self.client_profile_queue_guaranteed1_min_msg_burst = Some(client_profile_queue_guaranteed1_min_msg_burst);
  }

  pub fn with_client_profile_queue_guaranteed1_min_msg_burst(mut self, client_profile_queue_guaranteed1_min_msg_burst: i32) -> DmrClusterLink {
    self.client_profile_queue_guaranteed1_min_msg_burst = Some(client_profile_queue_guaranteed1_min_msg_burst);
    self
  }

  pub fn client_profile_queue_guaranteed1_min_msg_burst(&self) -> Option<&i32> {
    self.client_profile_queue_guaranteed1_min_msg_burst.as_ref()
  }

  pub fn reset_client_profile_queue_guaranteed1_min_msg_burst(&mut self) {
    self.client_profile_queue_guaranteed1_min_msg_burst = None;
  }

  pub fn set_client_profile_tcp_congestion_window_size(&mut self, client_profile_tcp_congestion_window_size: i64) {
    self.client_profile_tcp_congestion_window_size = Some(client_profile_tcp_congestion_window_size);
  }

  pub fn with_client_profile_tcp_congestion_window_size(mut self, client_profile_tcp_congestion_window_size: i64) -> DmrClusterLink {
    self.client_profile_tcp_congestion_window_size = Some(client_profile_tcp_congestion_window_size);
    self
  }

  pub fn client_profile_tcp_congestion_window_size(&self) -> Option<&i64> {
    self.client_profile_tcp_congestion_window_size.as_ref()
  }

  pub fn reset_client_profile_tcp_congestion_window_size(&mut self) {
    self.client_profile_tcp_congestion_window_size = None;
  }

  pub fn set_client_profile_tcp_keepalive_count(&mut self, client_profile_tcp_keepalive_count: i64) {
    self.client_profile_tcp_keepalive_count = Some(client_profile_tcp_keepalive_count);
  }

  pub fn with_client_profile_tcp_keepalive_count(mut self, client_profile_tcp_keepalive_count: i64) -> DmrClusterLink {
    self.client_profile_tcp_keepalive_count = Some(client_profile_tcp_keepalive_count);
    self
  }

  pub fn client_profile_tcp_keepalive_count(&self) -> Option<&i64> {
    self.client_profile_tcp_keepalive_count.as_ref()
  }

  pub fn reset_client_profile_tcp_keepalive_count(&mut self) {
    self.client_profile_tcp_keepalive_count = None;
  }

  pub fn set_client_profile_tcp_keepalive_idle_time(&mut self, client_profile_tcp_keepalive_idle_time: i64) {
    self.client_profile_tcp_keepalive_idle_time = Some(client_profile_tcp_keepalive_idle_time);
  }

  pub fn with_client_profile_tcp_keepalive_idle_time(mut self, client_profile_tcp_keepalive_idle_time: i64) -> DmrClusterLink {
    self.client_profile_tcp_keepalive_idle_time = Some(client_profile_tcp_keepalive_idle_time);
    self
  }

  pub fn client_profile_tcp_keepalive_idle_time(&self) -> Option<&i64> {
    self.client_profile_tcp_keepalive_idle_time.as_ref()
  }

  pub fn reset_client_profile_tcp_keepalive_idle_time(&mut self) {
    self.client_profile_tcp_keepalive_idle_time = None;
  }

  pub fn set_client_profile_tcp_keepalive_interval(&mut self, client_profile_tcp_keepalive_interval: i64) {
    self.client_profile_tcp_keepalive_interval = Some(client_profile_tcp_keepalive_interval);
  }

  pub fn with_client_profile_tcp_keepalive_interval(mut self, client_profile_tcp_keepalive_interval: i64) -> DmrClusterLink {
    self.client_profile_tcp_keepalive_interval = Some(client_profile_tcp_keepalive_interval);
    self
  }

  pub fn client_profile_tcp_keepalive_interval(&self) -> Option<&i64> {
    self.client_profile_tcp_keepalive_interval.as_ref()
  }

  pub fn reset_client_profile_tcp_keepalive_interval(&mut self) {
    self.client_profile_tcp_keepalive_interval = None;
  }

  pub fn set_client_profile_tcp_max_segment_size(&mut self, client_profile_tcp_max_segment_size: i64) {
    self.client_profile_tcp_max_segment_size = Some(client_profile_tcp_max_segment_size);
  }

  pub fn with_client_profile_tcp_max_segment_size(mut self, client_profile_tcp_max_segment_size: i64) -> DmrClusterLink {
    self.client_profile_tcp_max_segment_size = Some(client_profile_tcp_max_segment_size);
    self
  }

  pub fn client_profile_tcp_max_segment_size(&self) -> Option<&i64> {
    self.client_profile_tcp_max_segment_size.as_ref()
  }

  pub fn reset_client_profile_tcp_max_segment_size(&mut self) {
    self.client_profile_tcp_max_segment_size = None;
  }

  pub fn set_client_profile_tcp_max_window_size(&mut self, client_profile_tcp_max_window_size: i64) {
    self.client_profile_tcp_max_window_size = Some(client_profile_tcp_max_window_size);
  }

  pub fn with_client_profile_tcp_max_window_size(mut self, client_profile_tcp_max_window_size: i64) -> DmrClusterLink {
    self.client_profile_tcp_max_window_size = Some(client_profile_tcp_max_window_size);
    self
  }

  pub fn client_profile_tcp_max_window_size(&self) -> Option<&i64> {
    self.client_profile_tcp_max_window_size.as_ref()
  }

  pub fn reset_client_profile_tcp_max_window_size(&mut self) {
    self.client_profile_tcp_max_window_size = None;
  }

  pub fn set_dmr_cluster_name(&mut self, dmr_cluster_name: String) {
    self.dmr_cluster_name = Some(dmr_cluster_name);
  }

  pub fn with_dmr_cluster_name(mut self, dmr_cluster_name: String) -> DmrClusterLink {
    self.dmr_cluster_name = Some(dmr_cluster_name);
    self
  }

  pub fn dmr_cluster_name(&self) -> Option<&String> {
    self.dmr_cluster_name.as_ref()
  }

  pub fn reset_dmr_cluster_name(&mut self) {
    self.dmr_cluster_name = None;
  }

  pub fn set_egress_flow_window_size(&mut self, egress_flow_window_size: i64) {
    self.egress_flow_window_size = Some(egress_flow_window_size);
  }

  pub fn with_egress_flow_window_size(mut self, egress_flow_window_size: i64) -> DmrClusterLink {
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

  pub fn with_enabled(mut self, enabled: bool) -> DmrClusterLink {
    self.enabled = Some(enabled);
    self
  }

  pub fn enabled(&self) -> Option<&bool> {
    self.enabled.as_ref()
  }

  pub fn reset_enabled(&mut self) {
    self.enabled = None;
  }

  pub fn set_initiator(&mut self, initiator: String) {
    self.initiator = Some(initiator);
  }

  pub fn with_initiator(mut self, initiator: String) -> DmrClusterLink {
    self.initiator = Some(initiator);
    self
  }

  pub fn initiator(&self) -> Option<&String> {
    self.initiator.as_ref()
  }

  pub fn reset_initiator(&mut self) {
    self.initiator = None;
  }

  pub fn set_queue_dead_msg_queue(&mut self, queue_dead_msg_queue: String) {
    self.queue_dead_msg_queue = Some(queue_dead_msg_queue);
  }

  pub fn with_queue_dead_msg_queue(mut self, queue_dead_msg_queue: String) -> DmrClusterLink {
    self.queue_dead_msg_queue = Some(queue_dead_msg_queue);
    self
  }

  pub fn queue_dead_msg_queue(&self) -> Option<&String> {
    self.queue_dead_msg_queue.as_ref()
  }

  pub fn reset_queue_dead_msg_queue(&mut self) {
    self.queue_dead_msg_queue = None;
  }

  pub fn set_queue_event_spool_usage_threshold(&mut self, queue_event_spool_usage_threshold: ::models::EventThreshold) {
    self.queue_event_spool_usage_threshold = Some(queue_event_spool_usage_threshold);
  }

  pub fn with_queue_event_spool_usage_threshold(mut self, queue_event_spool_usage_threshold: ::models::EventThreshold) -> DmrClusterLink {
    self.queue_event_spool_usage_threshold = Some(queue_event_spool_usage_threshold);
    self
  }

  pub fn queue_event_spool_usage_threshold(&self) -> Option<&::models::EventThreshold> {
    self.queue_event_spool_usage_threshold.as_ref()
  }

  pub fn reset_queue_event_spool_usage_threshold(&mut self) {
    self.queue_event_spool_usage_threshold = None;
  }

  pub fn set_queue_max_delivered_unacked_msgs_per_flow(&mut self, queue_max_delivered_unacked_msgs_per_flow: i64) {
    self.queue_max_delivered_unacked_msgs_per_flow = Some(queue_max_delivered_unacked_msgs_per_flow);
  }

  pub fn with_queue_max_delivered_unacked_msgs_per_flow(mut self, queue_max_delivered_unacked_msgs_per_flow: i64) -> DmrClusterLink {
    self.queue_max_delivered_unacked_msgs_per_flow = Some(queue_max_delivered_unacked_msgs_per_flow);
    self
  }

  pub fn queue_max_delivered_unacked_msgs_per_flow(&self) -> Option<&i64> {
    self.queue_max_delivered_unacked_msgs_per_flow.as_ref()
  }

  pub fn reset_queue_max_delivered_unacked_msgs_per_flow(&mut self) {
    self.queue_max_delivered_unacked_msgs_per_flow = None;
  }

  pub fn set_queue_max_msg_spool_usage(&mut self, queue_max_msg_spool_usage: i64) {
    self.queue_max_msg_spool_usage = Some(queue_max_msg_spool_usage);
  }

  pub fn with_queue_max_msg_spool_usage(mut self, queue_max_msg_spool_usage: i64) -> DmrClusterLink {
    self.queue_max_msg_spool_usage = Some(queue_max_msg_spool_usage);
    self
  }

  pub fn queue_max_msg_spool_usage(&self) -> Option<&i64> {
    self.queue_max_msg_spool_usage.as_ref()
  }

  pub fn reset_queue_max_msg_spool_usage(&mut self) {
    self.queue_max_msg_spool_usage = None;
  }

  pub fn set_queue_max_redelivery_count(&mut self, queue_max_redelivery_count: i64) {
    self.queue_max_redelivery_count = Some(queue_max_redelivery_count);
  }

  pub fn with_queue_max_redelivery_count(mut self, queue_max_redelivery_count: i64) -> DmrClusterLink {
    self.queue_max_redelivery_count = Some(queue_max_redelivery_count);
    self
  }

  pub fn queue_max_redelivery_count(&self) -> Option<&i64> {
    self.queue_max_redelivery_count.as_ref()
  }

  pub fn reset_queue_max_redelivery_count(&mut self) {
    self.queue_max_redelivery_count = None;
  }

  pub fn set_queue_max_ttl(&mut self, queue_max_ttl: i64) {
    self.queue_max_ttl = Some(queue_max_ttl);
  }

  pub fn with_queue_max_ttl(mut self, queue_max_ttl: i64) -> DmrClusterLink {
    self.queue_max_ttl = Some(queue_max_ttl);
    self
  }

  pub fn queue_max_ttl(&self) -> Option<&i64> {
    self.queue_max_ttl.as_ref()
  }

  pub fn reset_queue_max_ttl(&mut self) {
    self.queue_max_ttl = None;
  }

  pub fn set_queue_reject_msg_to_sender_on_discard_behavior(&mut self, queue_reject_msg_to_sender_on_discard_behavior: String) {
    self.queue_reject_msg_to_sender_on_discard_behavior = Some(queue_reject_msg_to_sender_on_discard_behavior);
  }

  pub fn with_queue_reject_msg_to_sender_on_discard_behavior(mut self, queue_reject_msg_to_sender_on_discard_behavior: String) -> DmrClusterLink {
    self.queue_reject_msg_to_sender_on_discard_behavior = Some(queue_reject_msg_to_sender_on_discard_behavior);
    self
  }

  pub fn queue_reject_msg_to_sender_on_discard_behavior(&self) -> Option<&String> {
    self.queue_reject_msg_to_sender_on_discard_behavior.as_ref()
  }

  pub fn reset_queue_reject_msg_to_sender_on_discard_behavior(&mut self) {
    self.queue_reject_msg_to_sender_on_discard_behavior = None;
  }

  pub fn set_queue_respect_ttl_enabled(&mut self, queue_respect_ttl_enabled: bool) {
    self.queue_respect_ttl_enabled = Some(queue_respect_ttl_enabled);
  }

  pub fn with_queue_respect_ttl_enabled(mut self, queue_respect_ttl_enabled: bool) -> DmrClusterLink {
    self.queue_respect_ttl_enabled = Some(queue_respect_ttl_enabled);
    self
  }

  pub fn queue_respect_ttl_enabled(&self) -> Option<&bool> {
    self.queue_respect_ttl_enabled.as_ref()
  }

  pub fn reset_queue_respect_ttl_enabled(&mut self) {
    self.queue_respect_ttl_enabled = None;
  }

  pub fn set_remote_node_name(&mut self, remote_node_name: String) {
    self.remote_node_name = Some(remote_node_name);
  }

  pub fn with_remote_node_name(mut self, remote_node_name: String) -> DmrClusterLink {
    self.remote_node_name = Some(remote_node_name);
    self
  }

  pub fn remote_node_name(&self) -> Option<&String> {
    self.remote_node_name.as_ref()
  }

  pub fn reset_remote_node_name(&mut self) {
    self.remote_node_name = None;
  }

  pub fn set_span(&mut self, span: String) {
    self.span = Some(span);
  }

  pub fn with_span(mut self, span: String) -> DmrClusterLink {
    self.span = Some(span);
    self
  }

  pub fn span(&self) -> Option<&String> {
    self.span.as_ref()
  }

  pub fn reset_span(&mut self) {
    self.span = None;
  }

  pub fn set_transport_compressed_enabled(&mut self, transport_compressed_enabled: bool) {
    self.transport_compressed_enabled = Some(transport_compressed_enabled);
  }

  pub fn with_transport_compressed_enabled(mut self, transport_compressed_enabled: bool) -> DmrClusterLink {
    self.transport_compressed_enabled = Some(transport_compressed_enabled);
    self
  }

  pub fn transport_compressed_enabled(&self) -> Option<&bool> {
    self.transport_compressed_enabled.as_ref()
  }

  pub fn reset_transport_compressed_enabled(&mut self) {
    self.transport_compressed_enabled = None;
  }

  pub fn set_transport_tls_enabled(&mut self, transport_tls_enabled: bool) {
    self.transport_tls_enabled = Some(transport_tls_enabled);
  }

  pub fn with_transport_tls_enabled(mut self, transport_tls_enabled: bool) -> DmrClusterLink {
    self.transport_tls_enabled = Some(transport_tls_enabled);
    self
  }

  pub fn transport_tls_enabled(&self) -> Option<&bool> {
    self.transport_tls_enabled.as_ref()
  }

  pub fn reset_transport_tls_enabled(&mut self) {
    self.transport_tls_enabled = None;
  }

}



