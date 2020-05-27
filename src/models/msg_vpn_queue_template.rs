/* 
 * SEMP (Solace Element Management Protocol)
 *
 * SEMP (starting in `v2`, see note 1) is a RESTful API for configuring, monitoring, and administering a Solace PubSub+ broker.  SEMP uses URIs to address manageable **resources** of the Solace PubSub+ broker. Resources are individual **objects**, **collections** of objects, or (exclusively in the action API) **actions**. This document applies to the following API:   API|Base Path|Purpose|Comments :---|:---|:---|:--- Configuration|/SEMP/v2/config|Reading and writing config state|See note 2    The following APIs are also available:   API|Base Path|Purpose|Comments :---|:---|:---|:--- Action|/SEMP/v2/action|Performing actions|See note 2 Monitoring|/SEMP/v2/monitor|Querying operational parameters|See note 2    Resources are always nouns, with individual objects being singular and collections being plural.  Objects within a collection are identified by an `obj-id`, which follows the collection name with the form `collection-name/obj-id`.  Actions within an object are identified by an `action-id`, which follows the object name with the form `obj-id/action-id`.  Some examples:  ``` /SEMP/v2/config/msgVpns                        ; MsgVpn collection /SEMP/v2/config/msgVpns/a                      ; MsgVpn object named \"a\" /SEMP/v2/config/msgVpns/a/queues               ; Queue collection in MsgVpn \"a\" /SEMP/v2/config/msgVpns/a/queues/b             ; Queue object named \"b\" in MsgVpn \"a\" /SEMP/v2/action/msgVpns/a/queues/b/startReplay ; Action that starts a replay on Queue \"b\" in MsgVpn \"a\" /SEMP/v2/monitor/msgVpns/a/clients             ; Client collection in MsgVpn \"a\" /SEMP/v2/monitor/msgVpns/a/clients/c           ; Client object named \"c\" in MsgVpn \"a\" ```  ## Collection Resources  Collections are unordered lists of objects (unless described as otherwise), and are described by JSON arrays. Each item in the array represents an object in the same manner as the individual object would normally be represented. In the configuration API, the creation of a new object is done through its collection resource.  ## Object and Action Resources  Objects are composed of attributes, actions, collections, and other objects. They are described by JSON objects as name/value pairs. The collections and actions of an object are not contained directly in the object's JSON content; rather the content includes an attribute containing a URI which points to the collections and actions. These contained resources must be managed through this URI. At a minimum, every object has one or more identifying attributes, and its own `uri` attribute which contains the URI pointing to itself.  Actions are also composed of attributes, and are described by JSON objects as name/value pairs. Unlike objects, however, they are not members of a collection and cannot be retrieved, only performed. Actions only exist in the action API.  Attributes in an object or action may have any (non-exclusively) of the following properties:   Property|Meaning|Comments :---|:---|:--- Identifying|Attribute is involved in unique identification of the object, and appears in its URI| Required|Attribute must be provided in the request| Read-Only|Attribute can only be read, not written|See note 3 Write-Only|Attribute can only be written, not read| Requires-Disable|Attribute can only be changed when object is disabled| Deprecated|Attribute is deprecated, and will disappear in the next SEMP version|    In some requests, certain attributes may only be provided in certain combinations with other attributes:   Relationship|Meaning :---|:--- Requires|Attribute may only be changed by a request if a particular attribute or combination of attributes is also provided in the request Conflicts|Attribute may only be provided in a request if a particular attribute or combination of attributes is not also provided in the request    ## HTTP Methods  The following HTTP methods manipulate resources in accordance with these general principles. Note that some methods are only used in certain APIs:   Method|Resource|Meaning|Request Body|Response Body|Missing Request Attributes :---|:---|:---|:---|:---|:--- POST|Collection|Create object|Initial attribute values|Object attributes and metadata|Set to default PUT|Object|Create or replace object|New attribute values|Object attributes and metadata|Set to default (but see note 4) PUT|Action|Performs action|Action arguments|Action metadata|N/A PATCH|Object|Update object|New attribute values|Object attributes and metadata|unchanged DELETE|Object|Delete object|Empty|Object metadata|N/A GET|Object|Get object|Empty|Object attributes and metadata|N/A GET|Collection|Get collection|Empty|Object attributes and collection metadata|N/A    ## Common Query Parameters  The following are some common query parameters that are supported by many method/URI combinations. Individual URIs may document additional parameters. Note that multiple query parameters can be used together in a single URI, separated by the ampersand character. For example:  ``` ; Request for the MsgVpns collection using two hypothetical query parameters ; \"q1\" and \"q2\" with values \"val1\" and \"val2\" respectively /SEMP/v2/config/msgVpns?q1=val1&q2=val2 ```  ### select  Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. Use this query parameter to limit the size of the returned data for each returned object, return only those fields that are desired, or exclude fields that are not desired.  The value of `select` is a comma-separated list of attribute names. If the list contains attribute names that are not prefaced by `-`, only those attributes are included in the response. If the list contains attribute names that are prefaced by `-`, those attributes are excluded from the response. If the list contains both types, then the difference of the first set of attributes and the second set of attributes is returned. If the list is empty (i.e. `select=`), no attributes are returned.  All attributes that are prefaced by `-` must follow all attributes that are not prefaced by `-`. In addition, each attribute name in the list must match at least one attribute in the object.  Names may include the `*` wildcard (zero or more characters). Nested attribute names are supported using periods (e.g. `parentName.childName`).  Some examples:  ``` ; List of all MsgVpn names /SEMP/v2/config/msgVpns?select=msgVpnName ; List of all MsgVpn and their attributes except for their names /SEMP/v2/config/msgVpns?select=-msgVpnName ; Authentication attributes of MsgVpn \"finance\" /SEMP/v2/config/msgVpns/finance?select=authentication* ; All attributes of MsgVpn \"finance\" except for authentication attributes /SEMP/v2/config/msgVpns/finance?select=-authentication* ; Access related attributes of Queue \"orderQ\" of MsgVpn \"finance\" /SEMP/v2/config/msgVpns/finance/queues/orderQ?select=owner,permission ```  ### where  Include in the response only objects where certain conditions are true. Use this query parameter to limit which objects are returned to those whose attribute values meet the given conditions.  The value of `where` is a comma-separated list of expressions. All expressions must be true for the object to be included in the response. Each expression takes the form:  ``` expression  = attribute-name OP value OP          = '==' | '!=' | '&lt;' | '&gt;' | '&lt;=' | '&gt;=' ```  `value` may be a number, string, `true`, or `false`, as appropriate for the type of `attribute-name`. Greater-than and less-than comparisons only work for numbers. A `*` in a string `value` is interpreted as a wildcard (zero or more characters). Some examples:  ``` ; Only enabled MsgVpns /SEMP/v2/config/msgVpns?where=enabled==true ; Only MsgVpns using basic non-LDAP authentication /SEMP/v2/config/msgVpns?where=authenticationBasicEnabled==true,authenticationBasicType!=ldap ; Only MsgVpns that allow more than 100 client connections /SEMP/v2/config/msgVpns?where=maxConnectionCount>100 ; Only MsgVpns with msgVpnName starting with \"B\": /SEMP/v2/config/msgVpns?where=msgVpnName==B* ```  ### count  Limit the count of objects in the response. This can be useful to limit the size of the response for large collections. The minimum value for `count` is `1` and the default is `10`. There is also a per-collection maximum value to limit request handling time. For example:  ``` ; Up to 25 MsgVpns /SEMP/v2/config/msgVpns?count=25 ```  ### cursor  The cursor, or position, for the next page of objects. Cursors are opaque data that should not be created or interpreted by SEMP clients, and should only be used as described below.  When a request is made for a collection and there may be additional objects available for retrieval that are not included in the initial response, the response will include a `cursorQuery` field containing a cursor. The value of this field can be specified in the `cursor` query parameter of a subsequent request to retrieve the next page of objects. For convenience, an appropriate URI is constructed automatically by the broker and included in the `nextPageUri` field of the response. This URI can be used directly to retrieve the next page of objects.  ## Notes  Note|Description :---:|:--- 1|This specification defines SEMP starting in \"v2\", and not the original SEMP \"v1\" interface. Request and response formats between \"v1\" and \"v2\" are entirely incompatible, although both protocols share a common port configuration on the Solace PubSub+ broker. They are differentiated by the initial portion of the URI path, one of either \"/SEMP/\" or \"/SEMP/v2/\" 2|This API is partially implemented. Only a subset of all objects are available. 3|Read-only attributes may appear in POST and PUT/PATCH requests. However, if a read-only attribute is not marked as identifying, it will be ignored during a PUT/PATCH. 4|For PUT, if the SEMP user is not authorized to modify the attribute, its value is left unchanged rather than set to default. In addition, the values of write-only attributes are not set to their defaults on a PUT. If the object does not exist, it is created first.  
 *
 * OpenAPI spec version: 2.16
 * Contact: support@solace.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MsgVpnQueueTemplate {
  /// The access type for delivering messages to consumer flows. The default value is `\"exclusive\"`. The allowed values and their meaning are:  <pre> \"exclusive\" - Exclusive delivery of messages to the first bound consumer flow. \"non-exclusive\" - Non-exclusive delivery of messages to all bound consumer flows in a round-robin fashion. </pre> 
  #[serde(rename = "accessType", skip_serializing_if="Option::is_none")]
  access_type: Option<String>,
  /// Enable or disable the propagation of consumer acknowledgements (ACKs) received on the active replication Message VPN to the standby replication Message VPN. The default value is `true`.
  #[serde(rename = "consumerAckPropagationEnabled", skip_serializing_if="Option::is_none")]
  consumer_ack_propagation_enabled: Option<bool>,
  /// The name of the Dead Message Queue (DMQ). The default value is `\"#DEAD_MSG_QUEUE\"`.
  #[serde(rename = "deadMsgQueue", skip_serializing_if="Option::is_none")]
  dead_msg_queue: Option<String>,
  /// Controls the durability of queues created from this template. If non-durable, the created queue will be non-durable, regardless of the specified durability. If none, the created queue will have the requested durability. The default value is `\"none\"`. The allowed values and their meaning are:  <pre> \"none\" - The durability of the endpoint will be as requested on create. \"non-durable\" - The durability of the created queue will be non-durable, regardless of what was requested. </pre> 
  #[serde(rename = "durabilityOverride", skip_serializing_if="Option::is_none")]
  durability_override: Option<String>,
  #[serde(rename = "eventBindCountThreshold", skip_serializing_if="Option::is_none")]
  event_bind_count_threshold: Option<::models::EventThreshold>,
  #[serde(rename = "eventMsgSpoolUsageThreshold", skip_serializing_if="Option::is_none")]
  event_msg_spool_usage_threshold: Option<::models::EventThreshold>,
  #[serde(rename = "eventRejectLowPriorityMsgLimitThreshold", skip_serializing_if="Option::is_none")]
  event_reject_low_priority_msg_limit_threshold: Option<::models::EventThreshold>,
  /// The maximum number of consumer flows that can bind. The default value is `1000`.
  #[serde(rename = "maxBindCount", skip_serializing_if="Option::is_none")]
  max_bind_count: Option<i64>,
  /// The maximum number of messages delivered but not acknowledged per flow. The default value is `10000`.
  #[serde(rename = "maxDeliveredUnackedMsgsPerFlow", skip_serializing_if="Option::is_none")]
  max_delivered_unacked_msgs_per_flow: Option<i64>,
  /// The maximum message size allowed, in bytes (B). The default value is `10000000`.
  #[serde(rename = "maxMsgSize", skip_serializing_if="Option::is_none")]
  max_msg_size: Option<i32>,
  /// The maximum message spool usage allowed, in megabytes (MB). A value of 0 only allows spooling of the last message received and disables quota checking. The default value is `4000`.
  #[serde(rename = "maxMsgSpoolUsage", skip_serializing_if="Option::is_none")]
  max_msg_spool_usage: Option<i64>,
  /// The maximum number of message redelivery attempts that will occur prior to the message being discarded or moved to the DMQ. A value of 0 means to retry forever. The default value is `0`.
  #[serde(rename = "maxRedeliveryCount", skip_serializing_if="Option::is_none")]
  max_redelivery_count: Option<i64>,
  /// The maximum time in seconds a message can stay in a Queue when `respectTtlEnabled` is `\"true\"`. A message expires when the lesser of the sender assigned time-to-live (TTL) in the message and the `maxTtl` configured for the Queue, is exceeded. A value of 0 disables expiry. The default value is `0`.
  #[serde(rename = "maxTtl", skip_serializing_if="Option::is_none")]
  max_ttl: Option<i64>,
  /// The name of the Message VPN.
  #[serde(rename = "msgVpnName", skip_serializing_if="Option::is_none")]
  msg_vpn_name: Option<String>,
  /// The permission level for all consumers, excluding the owner. The default value is `\"no-access\"`. The allowed values and their meaning are:  <pre> \"no-access\" - Disallows all access. \"read-only\" - Read-only access to the messages. \"consume\" - Consume (read and remove) messages. \"modify-topic\" - Consume messages or modify the topic/selector. \"delete\" - Consume messages, modify the topic/selector or delete the Client created endpoint altogether. </pre> 
  #[serde(rename = "permission", skip_serializing_if="Option::is_none")]
  permission: Option<String>,
  /// A wildcardable pattern used to determine which Queues use settings from this Template. Two different wildcards are supported: * and >. Similar to topic filters or subscription patterns, a > matches anything (but only when used at the end), and a * matches zero or more characters but never a slash (/). A > is only a wildcard when used at the end, after a /. A * is only allowed at the end, after a slash (/). The default value is `\"\"`.
  #[serde(rename = "queueNameFilter", skip_serializing_if="Option::is_none")]
  queue_name_filter: Option<String>,
  /// The name of the Queue Template.
  #[serde(rename = "queueTemplateName", skip_serializing_if="Option::is_none")]
  queue_template_name: Option<String>,
  /// Enable or disable the checking of low priority messages against the `rejectLowPriorityMsgLimit`. This may only be enabled if `rejectMsgToSenderOnDiscardBehavior` does not have a value of `\"never\"`. The default value is `false`.
  #[serde(rename = "rejectLowPriorityMsgEnabled", skip_serializing_if="Option::is_none")]
  reject_low_priority_msg_enabled: Option<bool>,
  /// The number of messages of any priority above which low priority messages are not admitted but higher priority messages are allowed. The default value is `0`.
  #[serde(rename = "rejectLowPriorityMsgLimit", skip_serializing_if="Option::is_none")]
  reject_low_priority_msg_limit: Option<i64>,
  /// Determines when to return negative acknowledgements (NACKs) to sending clients on message discards. Note that NACKs prevent the message from being delivered to any destination and Transacted Session commits to fail. The default value is `\"when-queue-enabled\"`. The allowed values and their meaning are:  <pre> \"always\" - Always return a negative acknowledgment (NACK) to the sending client on message discard. \"when-queue-enabled\" - Only return a negative acknowledgment (NACK) to the sending client on message discard when the Queue is enabled. \"never\" - Never return a negative acknowledgment (NACK) to the sending client on message discard. </pre> 
  #[serde(rename = "rejectMsgToSenderOnDiscardBehavior", skip_serializing_if="Option::is_none")]
  reject_msg_to_sender_on_discard_behavior: Option<String>,
  /// Enable or disable the respecting of message priority. When enabled, messages are delivered in priority order, from 9 (highest) to 0 (lowest). The default value is `false`.
  #[serde(rename = "respectMsgPriorityEnabled", skip_serializing_if="Option::is_none")]
  respect_msg_priority_enabled: Option<bool>,
  /// Enable or disable the respecting of the time-to-live (TTL) for messages. When enabled, expired messages are discarded or moved to the DMQ. The default value is `false`.
  #[serde(rename = "respectTtlEnabled", skip_serializing_if="Option::is_none")]
  respect_ttl_enabled: Option<bool>
}

impl MsgVpnQueueTemplate {
  pub fn new() -> MsgVpnQueueTemplate {
    MsgVpnQueueTemplate {
      access_type: None,
      consumer_ack_propagation_enabled: None,
      dead_msg_queue: None,
      durability_override: None,
      event_bind_count_threshold: None,
      event_msg_spool_usage_threshold: None,
      event_reject_low_priority_msg_limit_threshold: None,
      max_bind_count: None,
      max_delivered_unacked_msgs_per_flow: None,
      max_msg_size: None,
      max_msg_spool_usage: None,
      max_redelivery_count: None,
      max_ttl: None,
      msg_vpn_name: None,
      permission: None,
      queue_name_filter: None,
      queue_template_name: None,
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

  pub fn with_access_type(mut self, access_type: String) -> MsgVpnQueueTemplate {
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

  pub fn with_consumer_ack_propagation_enabled(mut self, consumer_ack_propagation_enabled: bool) -> MsgVpnQueueTemplate {
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

  pub fn with_dead_msg_queue(mut self, dead_msg_queue: String) -> MsgVpnQueueTemplate {
    self.dead_msg_queue = Some(dead_msg_queue);
    self
  }

  pub fn dead_msg_queue(&self) -> Option<&String> {
    self.dead_msg_queue.as_ref()
  }

  pub fn reset_dead_msg_queue(&mut self) {
    self.dead_msg_queue = None;
  }

  pub fn set_durability_override(&mut self, durability_override: String) {
    self.durability_override = Some(durability_override);
  }

  pub fn with_durability_override(mut self, durability_override: String) -> MsgVpnQueueTemplate {
    self.durability_override = Some(durability_override);
    self
  }

  pub fn durability_override(&self) -> Option<&String> {
    self.durability_override.as_ref()
  }

  pub fn reset_durability_override(&mut self) {
    self.durability_override = None;
  }

  pub fn set_event_bind_count_threshold(&mut self, event_bind_count_threshold: ::models::EventThreshold) {
    self.event_bind_count_threshold = Some(event_bind_count_threshold);
  }

  pub fn with_event_bind_count_threshold(mut self, event_bind_count_threshold: ::models::EventThreshold) -> MsgVpnQueueTemplate {
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

  pub fn with_event_msg_spool_usage_threshold(mut self, event_msg_spool_usage_threshold: ::models::EventThreshold) -> MsgVpnQueueTemplate {
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

  pub fn with_event_reject_low_priority_msg_limit_threshold(mut self, event_reject_low_priority_msg_limit_threshold: ::models::EventThreshold) -> MsgVpnQueueTemplate {
    self.event_reject_low_priority_msg_limit_threshold = Some(event_reject_low_priority_msg_limit_threshold);
    self
  }

  pub fn event_reject_low_priority_msg_limit_threshold(&self) -> Option<&::models::EventThreshold> {
    self.event_reject_low_priority_msg_limit_threshold.as_ref()
  }

  pub fn reset_event_reject_low_priority_msg_limit_threshold(&mut self) {
    self.event_reject_low_priority_msg_limit_threshold = None;
  }

  pub fn set_max_bind_count(&mut self, max_bind_count: i64) {
    self.max_bind_count = Some(max_bind_count);
  }

  pub fn with_max_bind_count(mut self, max_bind_count: i64) -> MsgVpnQueueTemplate {
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

  pub fn with_max_delivered_unacked_msgs_per_flow(mut self, max_delivered_unacked_msgs_per_flow: i64) -> MsgVpnQueueTemplate {
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

  pub fn with_max_msg_size(mut self, max_msg_size: i32) -> MsgVpnQueueTemplate {
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

  pub fn with_max_msg_spool_usage(mut self, max_msg_spool_usage: i64) -> MsgVpnQueueTemplate {
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

  pub fn with_max_redelivery_count(mut self, max_redelivery_count: i64) -> MsgVpnQueueTemplate {
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

  pub fn with_max_ttl(mut self, max_ttl: i64) -> MsgVpnQueueTemplate {
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

  pub fn with_msg_vpn_name(mut self, msg_vpn_name: String) -> MsgVpnQueueTemplate {
    self.msg_vpn_name = Some(msg_vpn_name);
    self
  }

  pub fn msg_vpn_name(&self) -> Option<&String> {
    self.msg_vpn_name.as_ref()
  }

  pub fn reset_msg_vpn_name(&mut self) {
    self.msg_vpn_name = None;
  }

  pub fn set_permission(&mut self, permission: String) {
    self.permission = Some(permission);
  }

  pub fn with_permission(mut self, permission: String) -> MsgVpnQueueTemplate {
    self.permission = Some(permission);
    self
  }

  pub fn permission(&self) -> Option<&String> {
    self.permission.as_ref()
  }

  pub fn reset_permission(&mut self) {
    self.permission = None;
  }

  pub fn set_queue_name_filter(&mut self, queue_name_filter: String) {
    self.queue_name_filter = Some(queue_name_filter);
  }

  pub fn with_queue_name_filter(mut self, queue_name_filter: String) -> MsgVpnQueueTemplate {
    self.queue_name_filter = Some(queue_name_filter);
    self
  }

  pub fn queue_name_filter(&self) -> Option<&String> {
    self.queue_name_filter.as_ref()
  }

  pub fn reset_queue_name_filter(&mut self) {
    self.queue_name_filter = None;
  }

  pub fn set_queue_template_name(&mut self, queue_template_name: String) {
    self.queue_template_name = Some(queue_template_name);
  }

  pub fn with_queue_template_name(mut self, queue_template_name: String) -> MsgVpnQueueTemplate {
    self.queue_template_name = Some(queue_template_name);
    self
  }

  pub fn queue_template_name(&self) -> Option<&String> {
    self.queue_template_name.as_ref()
  }

  pub fn reset_queue_template_name(&mut self) {
    self.queue_template_name = None;
  }

  pub fn set_reject_low_priority_msg_enabled(&mut self, reject_low_priority_msg_enabled: bool) {
    self.reject_low_priority_msg_enabled = Some(reject_low_priority_msg_enabled);
  }

  pub fn with_reject_low_priority_msg_enabled(mut self, reject_low_priority_msg_enabled: bool) -> MsgVpnQueueTemplate {
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

  pub fn with_reject_low_priority_msg_limit(mut self, reject_low_priority_msg_limit: i64) -> MsgVpnQueueTemplate {
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

  pub fn with_reject_msg_to_sender_on_discard_behavior(mut self, reject_msg_to_sender_on_discard_behavior: String) -> MsgVpnQueueTemplate {
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

  pub fn with_respect_msg_priority_enabled(mut self, respect_msg_priority_enabled: bool) -> MsgVpnQueueTemplate {
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

  pub fn with_respect_ttl_enabled(mut self, respect_ttl_enabled: bool) -> MsgVpnQueueTemplate {
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



