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
pub struct MsgVpnQueue {
  /// The access type for delivering messages to consumer flows bound to the Queue. The default value is `\"exclusive\"`. The allowed values and their meaning are:  <pre> \"exclusive\" - Exclusive delivery of messages to the first bound consumer flow. \"non-exclusive\" - Non-exclusive delivery of messages to all bound consumer flows in a round-robin fashion. </pre> 
  #[serde(rename = "accessType", skip_serializing_if="Option::is_none")]
  access_type: Option<String>,
  /// Enable or disable the propagation of consumer acknowledgements (ACKs) received on the active replication Message VPN to the standby replication Message VPN. The default value is `true`.
  #[serde(rename = "consumerAckPropagationEnabled", skip_serializing_if="Option::is_none")]
  consumer_ack_propagation_enabled: Option<bool>,
  /// The name of the Dead Message Queue (DMQ) used by the Queue. The default value is `\"#DEAD_MSG_QUEUE\"`.
  #[serde(rename = "deadMsgQueue", skip_serializing_if="Option::is_none")]
  dead_msg_queue: Option<String>,
  /// Enable or disable the ability for client applications to query the message delivery count of messages received from the Queue. This is a controlled availability feature. Please contact Solace to find out if this feature is supported for your use case. The default value is `false`. Available since 2.19.
  #[serde(rename = "deliveryCountEnabled", skip_serializing_if="Option::is_none")]
  delivery_count_enabled: Option<bool>,
  /// Enable or disable the transmission of messages from the Queue. The default value is `false`.
  #[serde(rename = "egressEnabled", skip_serializing_if="Option::is_none")]
  egress_enabled: Option<bool>,
  #[serde(rename = "eventBindCountThreshold", skip_serializing_if="Option::is_none")]
  event_bind_count_threshold: Option<::models::EventThreshold>,
  #[serde(rename = "eventMsgSpoolUsageThreshold", skip_serializing_if="Option::is_none")]
  event_msg_spool_usage_threshold: Option<::models::EventThreshold>,
  #[serde(rename = "eventRejectLowPriorityMsgLimitThreshold", skip_serializing_if="Option::is_none")]
  event_reject_low_priority_msg_limit_threshold: Option<::models::EventThreshold>,
  /// Enable or disable the reception of messages to the Queue. The default value is `false`.
  #[serde(rename = "ingressEnabled", skip_serializing_if="Option::is_none")]
  ingress_enabled: Option<bool>,
  /// The maximum number of consumer flows that can bind to the Queue. The default value is `1000`.
  #[serde(rename = "maxBindCount", skip_serializing_if="Option::is_none")]
  max_bind_count: Option<i64>,
  /// The maximum number of messages delivered but not acknowledged per flow for the Queue. The default value is `10000`.
  #[serde(rename = "maxDeliveredUnackedMsgsPerFlow", skip_serializing_if="Option::is_none")]
  max_delivered_unacked_msgs_per_flow: Option<i64>,
  /// The maximum message size allowed in the Queue, in bytes (B). The default value is `10000000`.
  #[serde(rename = "maxMsgSize", skip_serializing_if="Option::is_none")]
  max_msg_size: Option<i32>,
  /// The maximum message spool usage allowed by the Queue, in megabytes (MB). A value of 0 only allows spooling of the last message received and disables quota checking. The default value is `4000`.
  #[serde(rename = "maxMsgSpoolUsage", skip_serializing_if="Option::is_none")]
  max_msg_spool_usage: Option<i64>,
  /// The maximum number of times the Queue will attempt redelivery of a message prior to it being discarded or moved to the DMQ. A value of 0 means to retry forever. The default value is `0`.
  #[serde(rename = "maxRedeliveryCount", skip_serializing_if="Option::is_none")]
  max_redelivery_count: Option<i64>,
  /// The maximum time in seconds a message can stay in the Queue when `respectTtlEnabled` is `\"true\"`. A message expires when the lesser of the sender assigned time-to-live (TTL) in the message and the `maxTtl` configured for the Queue, is exceeded. A value of 0 disables expiry. The default value is `0`.
  #[serde(rename = "maxTtl", skip_serializing_if="Option::is_none")]
  max_ttl: Option<i64>,
  /// The name of the Message VPN.
  #[serde(rename = "msgVpnName", skip_serializing_if="Option::is_none")]
  msg_vpn_name: Option<String>,
  /// The Client Username that owns the Queue and has permission equivalent to `\"delete\"`. The default value is `\"\"`.
  #[serde(rename = "owner", skip_serializing_if="Option::is_none")]
  owner: Option<String>,
  /// The permission level for all consumers of the Queue, excluding the owner. The default value is `\"no-access\"`. The allowed values and their meaning are:  <pre> \"no-access\" - Disallows all access. \"read-only\" - Read-only access to the messages. \"consume\" - Consume (read and remove) messages. \"modify-topic\" - Consume messages or modify the topic/selector. \"delete\" - Consume messages, modify the topic/selector or delete the Client created endpoint altogether. </pre> 
  #[serde(rename = "permission", skip_serializing_if="Option::is_none")]
  permission: Option<String>,
  /// The name of the Queue.
  #[serde(rename = "queueName", skip_serializing_if="Option::is_none")]
  queue_name: Option<String>,
  /// Enable or disable message redelivery. When enabled, the number of redelivery attempts is controlled by maxRedeliveryCount. When disabled, the message will never be delivered from the queue more than once. The default value is `true`. Available since 2.18.
  #[serde(rename = "redeliveryEnabled", skip_serializing_if="Option::is_none")]
  redelivery_enabled: Option<bool>,
  /// Enable or disable the checking of low priority messages against the `rejectLowPriorityMsgLimit`. This may only be enabled if `rejectMsgToSenderOnDiscardBehavior` does not have a value of `\"never\"`. The default value is `false`.
  #[serde(rename = "rejectLowPriorityMsgEnabled", skip_serializing_if="Option::is_none")]
  reject_low_priority_msg_enabled: Option<bool>,
  /// The number of messages of any priority in the Queue above which low priority messages are not admitted but higher priority messages are allowed. The default value is `0`.
  #[serde(rename = "rejectLowPriorityMsgLimit", skip_serializing_if="Option::is_none")]
  reject_low_priority_msg_limit: Option<i64>,
  /// Determines when to return negative acknowledgements (NACKs) to sending clients on message discards. Note that NACKs cause the message to not be delivered to any destination and Transacted Session commits to fail. The default value is `\"when-queue-enabled\"`. The allowed values and their meaning are:  <pre> \"always\" - Always return a negative acknowledgment (NACK) to the sending client on message discard. \"when-queue-enabled\" - Only return a negative acknowledgment (NACK) to the sending client on message discard when the Queue is enabled. \"never\" - Never return a negative acknowledgment (NACK) to the sending client on message discard. </pre> 
  #[serde(rename = "rejectMsgToSenderOnDiscardBehavior", skip_serializing_if="Option::is_none")]
  reject_msg_to_sender_on_discard_behavior: Option<String>,
  /// Enable or disable the respecting of message priority. When enabled, messages contained in the Queue are delivered in priority order, from 9 (highest) to 0 (lowest). MQTT queues do not support enabling message priority. The default value is `false`. Available since 2.8.
  #[serde(rename = "respectMsgPriorityEnabled", skip_serializing_if="Option::is_none")]
  respect_msg_priority_enabled: Option<bool>,
  /// Enable or disable the respecting of the time-to-live (TTL) for messages in the Queue. When enabled, expired messages are discarded or moved to the DMQ. The default value is `false`.
  #[serde(rename = "respectTtlEnabled", skip_serializing_if="Option::is_none")]
  respect_ttl_enabled: Option<bool>
}

impl MsgVpnQueue {
  pub fn new() -> MsgVpnQueue {
    MsgVpnQueue {
      access_type: None,
      consumer_ack_propagation_enabled: None,
      dead_msg_queue: None,
      delivery_count_enabled: None,
      egress_enabled: None,
      event_bind_count_threshold: None,
      event_msg_spool_usage_threshold: None,
      event_reject_low_priority_msg_limit_threshold: None,
      ingress_enabled: None,
      max_bind_count: None,
      max_delivered_unacked_msgs_per_flow: None,
      max_msg_size: None,
      max_msg_spool_usage: None,
      max_redelivery_count: None,
      max_ttl: None,
      msg_vpn_name: None,
      owner: None,
      permission: None,
      queue_name: None,
      redelivery_enabled: None,
      reject_low_priority_msg_enabled: None,
      reject_low_priority_msg_limit: None,
      reject_msg_to_sender_on_discard_behavior: None,
      respect_msg_priority_enabled: None,
      respect_ttl_enabled: None
    }
  }

  pub fn set_access_type(&mut self, access_type: String) {
    self.access_type = Some(access_type);
  }

  pub fn with_access_type(mut self, access_type: String) -> MsgVpnQueue {
    self.access_type = Some(access_type);
    self
  }

  pub fn access_type(&self) -> Option<&String> {
    self.access_type.as_ref()
  }

  pub fn reset_access_type(&mut self) {
    self.access_type = None;
  }

  pub fn set_consumer_ack_propagation_enabled(&mut self, consumer_ack_propagation_enabled: bool) {
    self.consumer_ack_propagation_enabled = Some(consumer_ack_propagation_enabled);
  }

  pub fn with_consumer_ack_propagation_enabled(mut self, consumer_ack_propagation_enabled: bool) -> MsgVpnQueue {
    self.consumer_ack_propagation_enabled = Some(consumer_ack_propagation_enabled);
    self
  }

  pub fn consumer_ack_propagation_enabled(&self) -> Option<&bool> {
    self.consumer_ack_propagation_enabled.as_ref()
  }

  pub fn reset_consumer_ack_propagation_enabled(&mut self) {
    self.consumer_ack_propagation_enabled = None;
  }

  pub fn set_dead_msg_queue(&mut self, dead_msg_queue: String) {
    self.dead_msg_queue = Some(dead_msg_queue);
  }

  pub fn with_dead_msg_queue(mut self, dead_msg_queue: String) -> MsgVpnQueue {
    self.dead_msg_queue = Some(dead_msg_queue);
    self
  }

  pub fn dead_msg_queue(&self) -> Option<&String> {
    self.dead_msg_queue.as_ref()
  }

  pub fn reset_dead_msg_queue(&mut self) {
    self.dead_msg_queue = None;
  }

  pub fn set_delivery_count_enabled(&mut self, delivery_count_enabled: bool) {
    self.delivery_count_enabled = Some(delivery_count_enabled);
  }

  pub fn with_delivery_count_enabled(mut self, delivery_count_enabled: bool) -> MsgVpnQueue {
    self.delivery_count_enabled = Some(delivery_count_enabled);
    self
  }

  pub fn delivery_count_enabled(&self) -> Option<&bool> {
    self.delivery_count_enabled.as_ref()
  }

  pub fn reset_delivery_count_enabled(&mut self) {
    self.delivery_count_enabled = None;
  }

  pub fn set_egress_enabled(&mut self, egress_enabled: bool) {
    self.egress_enabled = Some(egress_enabled);
  }

  pub fn with_egress_enabled(mut self, egress_enabled: bool) -> MsgVpnQueue {
    self.egress_enabled = Some(egress_enabled);
    self
  }

  pub fn egress_enabled(&self) -> Option<&bool> {
    self.egress_enabled.as_ref()
  }

  pub fn reset_egress_enabled(&mut self) {
    self.egress_enabled = None;
  }

  pub fn set_event_bind_count_threshold(&mut self, event_bind_count_threshold: ::models::EventThreshold) {
    self.event_bind_count_threshold = Some(event_bind_count_threshold);
  }

  pub fn with_event_bind_count_threshold(mut self, event_bind_count_threshold: ::models::EventThreshold) -> MsgVpnQueue {
    self.event_bind_count_threshold = Some(event_bind_count_threshold);
    self
  }

  pub fn event_bind_count_threshold(&self) -> Option<&::models::EventThreshold> {
    self.event_bind_count_threshold.as_ref()
  }

  pub fn reset_event_bind_count_threshold(&mut self) {
    self.event_bind_count_threshold = None;
  }

  pub fn set_event_msg_spool_usage_threshold(&mut self, event_msg_spool_usage_threshold: ::models::EventThreshold) {
    self.event_msg_spool_usage_threshold = Some(event_msg_spool_usage_threshold);
  }

  pub fn with_event_msg_spool_usage_threshold(mut self, event_msg_spool_usage_threshold: ::models::EventThreshold) -> MsgVpnQueue {
    self.event_msg_spool_usage_threshold = Some(event_msg_spool_usage_threshold);
    self
  }

  pub fn event_msg_spool_usage_threshold(&self) -> Option<&::models::EventThreshold> {
    self.event_msg_spool_usage_threshold.as_ref()
  }

  pub fn reset_event_msg_spool_usage_threshold(&mut self) {
    self.event_msg_spool_usage_threshold = None;
  }

  pub fn set_event_reject_low_priority_msg_limit_threshold(&mut self, event_reject_low_priority_msg_limit_threshold: ::models::EventThreshold) {
    self.event_reject_low_priority_msg_limit_threshold = Some(event_reject_low_priority_msg_limit_threshold);
  }

  pub fn with_event_reject_low_priority_msg_limit_threshold(mut self, event_reject_low_priority_msg_limit_threshold: ::models::EventThreshold) -> MsgVpnQueue {
    self.event_reject_low_priority_msg_limit_threshold = Some(event_reject_low_priority_msg_limit_threshold);
    self
  }

  pub fn event_reject_low_priority_msg_limit_threshold(&self) -> Option<&::models::EventThreshold> {
    self.event_reject_low_priority_msg_limit_threshold.as_ref()
  }

  pub fn reset_event_reject_low_priority_msg_limit_threshold(&mut self) {
    self.event_reject_low_priority_msg_limit_threshold = None;
  }

  pub fn set_ingress_enabled(&mut self, ingress_enabled: bool) {
    self.ingress_enabled = Some(ingress_enabled);
  }

  pub fn with_ingress_enabled(mut self, ingress_enabled: bool) -> MsgVpnQueue {
    self.ingress_enabled = Some(ingress_enabled);
    self
  }

  pub fn ingress_enabled(&self) -> Option<&bool> {
    self.ingress_enabled.as_ref()
  }

  pub fn reset_ingress_enabled(&mut self) {
    self.ingress_enabled = None;
  }

  pub fn set_max_bind_count(&mut self, max_bind_count: i64) {
    self.max_bind_count = Some(max_bind_count);
  }

  pub fn with_max_bind_count(mut self, max_bind_count: i64) -> MsgVpnQueue {
    self.max_bind_count = Some(max_bind_count);
    self
  }

  pub fn max_bind_count(&self) -> Option<&i64> {
    self.max_bind_count.as_ref()
  }

  pub fn reset_max_bind_count(&mut self) {
    self.max_bind_count = None;
  }

  pub fn set_max_delivered_unacked_msgs_per_flow(&mut self, max_delivered_unacked_msgs_per_flow: i64) {
    self.max_delivered_unacked_msgs_per_flow = Some(max_delivered_unacked_msgs_per_flow);
  }

  pub fn with_max_delivered_unacked_msgs_per_flow(mut self, max_delivered_unacked_msgs_per_flow: i64) -> MsgVpnQueue {
    self.max_delivered_unacked_msgs_per_flow = Some(max_delivered_unacked_msgs_per_flow);
    self
  }

  pub fn max_delivered_unacked_msgs_per_flow(&self) -> Option<&i64> {
    self.max_delivered_unacked_msgs_per_flow.as_ref()
  }

  pub fn reset_max_delivered_unacked_msgs_per_flow(&mut self) {
    self.max_delivered_unacked_msgs_per_flow = None;
  }

  pub fn set_max_msg_size(&mut self, max_msg_size: i32) {
    self.max_msg_size = Some(max_msg_size);
  }

  pub fn with_max_msg_size(mut self, max_msg_size: i32) -> MsgVpnQueue {
    self.max_msg_size = Some(max_msg_size);
    self
  }

  pub fn max_msg_size(&self) -> Option<&i32> {
    self.max_msg_size.as_ref()
  }

  pub fn reset_max_msg_size(&mut self) {
    self.max_msg_size = None;
  }

  pub fn set_max_msg_spool_usage(&mut self, max_msg_spool_usage: i64) {
    self.max_msg_spool_usage = Some(max_msg_spool_usage);
  }

  pub fn with_max_msg_spool_usage(mut self, max_msg_spool_usage: i64) -> MsgVpnQueue {
    self.max_msg_spool_usage = Some(max_msg_spool_usage);
    self
  }

  pub fn max_msg_spool_usage(&self) -> Option<&i64> {
    self.max_msg_spool_usage.as_ref()
  }

  pub fn reset_max_msg_spool_usage(&mut self) {
    self.max_msg_spool_usage = None;
  }

  pub fn set_max_redelivery_count(&mut self, max_redelivery_count: i64) {
    self.max_redelivery_count = Some(max_redelivery_count);
  }

  pub fn with_max_redelivery_count(mut self, max_redelivery_count: i64) -> MsgVpnQueue {
    self.max_redelivery_count = Some(max_redelivery_count);
    self
  }

  pub fn max_redelivery_count(&self) -> Option<&i64> {
    self.max_redelivery_count.as_ref()
  }

  pub fn reset_max_redelivery_count(&mut self) {
    self.max_redelivery_count = None;
  }

  pub fn set_max_ttl(&mut self, max_ttl: i64) {
    self.max_ttl = Some(max_ttl);
  }

  pub fn with_max_ttl(mut self, max_ttl: i64) -> MsgVpnQueue {
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

  pub fn with_msg_vpn_name(mut self, msg_vpn_name: String) -> MsgVpnQueue {
    self.msg_vpn_name = Some(msg_vpn_name);
    self
  }

  pub fn msg_vpn_name(&self) -> Option<&String> {
    self.msg_vpn_name.as_ref()
  }

  pub fn reset_msg_vpn_name(&mut self) {
    self.msg_vpn_name = None;
  }

  pub fn set_owner(&mut self, owner: String) {
    self.owner = Some(owner);
  }

  pub fn with_owner(mut self, owner: String) -> MsgVpnQueue {
    self.owner = Some(owner);
    self
  }

  pub fn owner(&self) -> Option<&String> {
    self.owner.as_ref()
  }

  pub fn reset_owner(&mut self) {
    self.owner = None;
  }

  pub fn set_permission(&mut self, permission: String) {
    self.permission = Some(permission);
  }

  pub fn with_permission(mut self, permission: String) -> MsgVpnQueue {
    self.permission = Some(permission);
    self
  }

  pub fn permission(&self) -> Option<&String> {
    self.permission.as_ref()
  }

  pub fn reset_permission(&mut self) {
    self.permission = None;
  }

  pub fn set_queue_name(&mut self, queue_name: String) {
    self.queue_name = Some(queue_name);
  }

  pub fn with_queue_name(mut self, queue_name: String) -> MsgVpnQueue {
    self.queue_name = Some(queue_name);
    self
  }

  pub fn queue_name(&self) -> Option<&String> {
    self.queue_name.as_ref()
  }

  pub fn reset_queue_name(&mut self) {
    self.queue_name = None;
  }

  pub fn set_redelivery_enabled(&mut self, redelivery_enabled: bool) {
    self.redelivery_enabled = Some(redelivery_enabled);
  }

  pub fn with_redelivery_enabled(mut self, redelivery_enabled: bool) -> MsgVpnQueue {
    self.redelivery_enabled = Some(redelivery_enabled);
    self
  }

  pub fn redelivery_enabled(&self) -> Option<&bool> {
    self.redelivery_enabled.as_ref()
  }

  pub fn reset_redelivery_enabled(&mut self) {
    self.redelivery_enabled = None;
  }

  pub fn set_reject_low_priority_msg_enabled(&mut self, reject_low_priority_msg_enabled: bool) {
    self.reject_low_priority_msg_enabled = Some(reject_low_priority_msg_enabled);
  }

  pub fn with_reject_low_priority_msg_enabled(mut self, reject_low_priority_msg_enabled: bool) -> MsgVpnQueue {
    self.reject_low_priority_msg_enabled = Some(reject_low_priority_msg_enabled);
    self
  }

  pub fn reject_low_priority_msg_enabled(&self) -> Option<&bool> {
    self.reject_low_priority_msg_enabled.as_ref()
  }

  pub fn reset_reject_low_priority_msg_enabled(&mut self) {
    self.reject_low_priority_msg_enabled = None;
  }

  pub fn set_reject_low_priority_msg_limit(&mut self, reject_low_priority_msg_limit: i64) {
    self.reject_low_priority_msg_limit = Some(reject_low_priority_msg_limit);
  }

  pub fn with_reject_low_priority_msg_limit(mut self, reject_low_priority_msg_limit: i64) -> MsgVpnQueue {
    self.reject_low_priority_msg_limit = Some(reject_low_priority_msg_limit);
    self
  }

  pub fn reject_low_priority_msg_limit(&self) -> Option<&i64> {
    self.reject_low_priority_msg_limit.as_ref()
  }

  pub fn reset_reject_low_priority_msg_limit(&mut self) {
    self.reject_low_priority_msg_limit = None;
  }

  pub fn set_reject_msg_to_sender_on_discard_behavior(&mut self, reject_msg_to_sender_on_discard_behavior: String) {
    self.reject_msg_to_sender_on_discard_behavior = Some(reject_msg_to_sender_on_discard_behavior);
  }

  pub fn with_reject_msg_to_sender_on_discard_behavior(mut self, reject_msg_to_sender_on_discard_behavior: String) -> MsgVpnQueue {
    self.reject_msg_to_sender_on_discard_behavior = Some(reject_msg_to_sender_on_discard_behavior);
    self
  }

  pub fn reject_msg_to_sender_on_discard_behavior(&self) -> Option<&String> {
    self.reject_msg_to_sender_on_discard_behavior.as_ref()
  }

  pub fn reset_reject_msg_to_sender_on_discard_behavior(&mut self) {
    self.reject_msg_to_sender_on_discard_behavior = None;
  }

  pub fn set_respect_msg_priority_enabled(&mut self, respect_msg_priority_enabled: bool) {
    self.respect_msg_priority_enabled = Some(respect_msg_priority_enabled);
  }

  pub fn with_respect_msg_priority_enabled(mut self, respect_msg_priority_enabled: bool) -> MsgVpnQueue {
    self.respect_msg_priority_enabled = Some(respect_msg_priority_enabled);
    self
  }

  pub fn respect_msg_priority_enabled(&self) -> Option<&bool> {
    self.respect_msg_priority_enabled.as_ref()
  }

  pub fn reset_respect_msg_priority_enabled(&mut self) {
    self.respect_msg_priority_enabled = None;
  }

  pub fn set_respect_ttl_enabled(&mut self, respect_ttl_enabled: bool) {
    self.respect_ttl_enabled = Some(respect_ttl_enabled);
  }

  pub fn with_respect_ttl_enabled(mut self, respect_ttl_enabled: bool) -> MsgVpnQueue {
    self.respect_ttl_enabled = Some(respect_ttl_enabled);
    self
  }

  pub fn respect_ttl_enabled(&self) -> Option<&bool> {
    self.respect_ttl_enabled.as_ref()
  }

  pub fn reset_respect_ttl_enabled(&mut self) {
    self.respect_ttl_enabled = None;
  }

}



