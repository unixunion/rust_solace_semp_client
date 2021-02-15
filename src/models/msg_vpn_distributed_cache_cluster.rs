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
pub struct MsgVpnDistributedCacheCluster {
  /// The name of the Distributed Cache.
  #[serde(rename = "cacheName", skip_serializing_if="Option::is_none")]
  cache_name: Option<String>,
  /// The name of the Cache Cluster.
  #[serde(rename = "clusterName", skip_serializing_if="Option::is_none")]
  cluster_name: Option<String>,
  /// Enable or disable deliver-to-one override for the Cache Cluster. The default value is `true`.
  #[serde(rename = "deliverToOneOverrideEnabled", skip_serializing_if="Option::is_none")]
  deliver_to_one_override_enabled: Option<bool>,
  /// Enable or disable the Cache Cluster. The default value is `false`.
  #[serde(rename = "enabled", skip_serializing_if="Option::is_none")]
  enabled: Option<bool>,
  #[serde(rename = "eventDataByteRateThreshold", skip_serializing_if="Option::is_none")]
  event_data_byte_rate_threshold: Option<::models::EventThresholdByValue>,
  #[serde(rename = "eventDataMsgRateThreshold", skip_serializing_if="Option::is_none")]
  event_data_msg_rate_threshold: Option<::models::EventThresholdByValue>,
  #[serde(rename = "eventMaxMemoryThreshold", skip_serializing_if="Option::is_none")]
  event_max_memory_threshold: Option<::models::EventThresholdByPercent>,
  #[serde(rename = "eventMaxTopicsThreshold", skip_serializing_if="Option::is_none")]
  event_max_topics_threshold: Option<::models::EventThresholdByPercent>,
  #[serde(rename = "eventRequestQueueDepthThreshold", skip_serializing_if="Option::is_none")]
  event_request_queue_depth_threshold: Option<::models::EventThresholdByPercent>,
  #[serde(rename = "eventRequestRateThreshold", skip_serializing_if="Option::is_none")]
  event_request_rate_threshold: Option<::models::EventThresholdByValue>,
  #[serde(rename = "eventResponseRateThreshold", skip_serializing_if="Option::is_none")]
  event_response_rate_threshold: Option<::models::EventThresholdByValue>,
  /// Enable or disable global caching for the Cache Cluster. When enabled, the Cache Instances will fetch topics from remote Home Cache Clusters when requested, and subscribe to those topics to cache them locally. When disabled, the Cache Instances will remove all subscriptions and cached messages for topics from remote Home Cache Clusters. The default value is `false`.
  #[serde(rename = "globalCachingEnabled", skip_serializing_if="Option::is_none")]
  global_caching_enabled: Option<bool>,
  /// The heartbeat interval, in seconds, used by the Cache Instances to monitor connectivity with the remote Home Cache Clusters. The default value is `3`.
  #[serde(rename = "globalCachingHeartbeat", skip_serializing_if="Option::is_none")]
  global_caching_heartbeat: Option<i64>,
  /// The topic lifetime, in seconds. If no client requests are received for a given global topic over the duration of the topic lifetime, then the Cache Instance will remove the subscription and cached messages for that topic. A value of 0 disables aging. The default value is `3600`.
  #[serde(rename = "globalCachingTopicLifetime", skip_serializing_if="Option::is_none")]
  global_caching_topic_lifetime: Option<i64>,
  /// The maximum memory usage, in megabytes (MB), for each Cache Instance in the Cache Cluster. The default value is `2048`.
  #[serde(rename = "maxMemory", skip_serializing_if="Option::is_none")]
  max_memory: Option<i64>,
  /// The maximum number of messages per topic for each Cache Instance in the Cache Cluster. When at the maximum, old messages are removed as new messages arrive. The default value is `1`.
  #[serde(rename = "maxMsgsPerTopic", skip_serializing_if="Option::is_none")]
  max_msgs_per_topic: Option<i64>,
  /// The maximum queue depth for cache requests received by the Cache Cluster. The default value is `100000`.
  #[serde(rename = "maxRequestQueueDepth", skip_serializing_if="Option::is_none")]
  max_request_queue_depth: Option<i64>,
  /// The maximum number of topics for each Cache Instance in the Cache Cluster. The default value is `2000000`.
  #[serde(rename = "maxTopicCount", skip_serializing_if="Option::is_none")]
  max_topic_count: Option<i64>,
  /// The message lifetime, in seconds. If a message remains cached for the duration of its lifetime, the Cache Instance will remove the message. A lifetime of 0 results in the message being retained indefinitely. The default value is `0`.
  #[serde(rename = "msgLifetime", skip_serializing_if="Option::is_none")]
  msg_lifetime: Option<i64>,
  /// The name of the Message VPN.
  #[serde(rename = "msgVpnName", skip_serializing_if="Option::is_none")]
  msg_vpn_name: Option<String>,
  /// Enable or disable the advertising, onto the message bus, of new topics learned by each Cache Instance in the Cache Cluster. The default value is `false`.
  #[serde(rename = "newTopicAdvertisementEnabled", skip_serializing_if="Option::is_none")]
  new_topic_advertisement_enabled: Option<bool>
}

impl MsgVpnDistributedCacheCluster {
  pub fn new() -> MsgVpnDistributedCacheCluster {
    MsgVpnDistributedCacheCluster {
      cache_name: None,
      cluster_name: None,
      deliver_to_one_override_enabled: None,
      enabled: None,
      event_data_byte_rate_threshold: None,
      event_data_msg_rate_threshold: None,
      event_max_memory_threshold: None,
      event_max_topics_threshold: None,
      event_request_queue_depth_threshold: None,
      event_request_rate_threshold: None,
      event_response_rate_threshold: None,
      global_caching_enabled: None,
      global_caching_heartbeat: None,
      global_caching_topic_lifetime: None,
      max_memory: None,
      max_msgs_per_topic: None,
      max_request_queue_depth: None,
      max_topic_count: None,
      msg_lifetime: None,
      msg_vpn_name: None,
      new_topic_advertisement_enabled: None
    }
  }

  pub fn set_cache_name(&mut self, cache_name: String) {
    self.cache_name = Some(cache_name);
  }

  pub fn with_cache_name(mut self, cache_name: String) -> MsgVpnDistributedCacheCluster {
    self.cache_name = Some(cache_name);
    self
  }

  pub fn cache_name(&self) -> Option<&String> {
    self.cache_name.as_ref()
  }

  pub fn reset_cache_name(&mut self) {
    self.cache_name = None;
  }

  pub fn set_cluster_name(&mut self, cluster_name: String) {
    self.cluster_name = Some(cluster_name);
  }

  pub fn with_cluster_name(mut self, cluster_name: String) -> MsgVpnDistributedCacheCluster {
    self.cluster_name = Some(cluster_name);
    self
  }

  pub fn cluster_name(&self) -> Option<&String> {
    self.cluster_name.as_ref()
  }

  pub fn reset_cluster_name(&mut self) {
    self.cluster_name = None;
  }

  pub fn set_deliver_to_one_override_enabled(&mut self, deliver_to_one_override_enabled: bool) {
    self.deliver_to_one_override_enabled = Some(deliver_to_one_override_enabled);
  }

  pub fn with_deliver_to_one_override_enabled(mut self, deliver_to_one_override_enabled: bool) -> MsgVpnDistributedCacheCluster {
    self.deliver_to_one_override_enabled = Some(deliver_to_one_override_enabled);
    self
  }

  pub fn deliver_to_one_override_enabled(&self) -> Option<&bool> {
    self.deliver_to_one_override_enabled.as_ref()
  }

  pub fn reset_deliver_to_one_override_enabled(&mut self) {
    self.deliver_to_one_override_enabled = None;
  }

  pub fn set_enabled(&mut self, enabled: bool) {
    self.enabled = Some(enabled);
  }

  pub fn with_enabled(mut self, enabled: bool) -> MsgVpnDistributedCacheCluster {
    self.enabled = Some(enabled);
    self
  }

  pub fn enabled(&self) -> Option<&bool> {
    self.enabled.as_ref()
  }

  pub fn reset_enabled(&mut self) {
    self.enabled = None;
  }

  pub fn set_event_data_byte_rate_threshold(&mut self, event_data_byte_rate_threshold: ::models::EventThresholdByValue) {
    self.event_data_byte_rate_threshold = Some(event_data_byte_rate_threshold);
  }

  pub fn with_event_data_byte_rate_threshold(mut self, event_data_byte_rate_threshold: ::models::EventThresholdByValue) -> MsgVpnDistributedCacheCluster {
    self.event_data_byte_rate_threshold = Some(event_data_byte_rate_threshold);
    self
  }

  pub fn event_data_byte_rate_threshold(&self) -> Option<&::models::EventThresholdByValue> {
    self.event_data_byte_rate_threshold.as_ref()
  }

  pub fn reset_event_data_byte_rate_threshold(&mut self) {
    self.event_data_byte_rate_threshold = None;
  }

  pub fn set_event_data_msg_rate_threshold(&mut self, event_data_msg_rate_threshold: ::models::EventThresholdByValue) {
    self.event_data_msg_rate_threshold = Some(event_data_msg_rate_threshold);
  }

  pub fn with_event_data_msg_rate_threshold(mut self, event_data_msg_rate_threshold: ::models::EventThresholdByValue) -> MsgVpnDistributedCacheCluster {
    self.event_data_msg_rate_threshold = Some(event_data_msg_rate_threshold);
    self
  }

  pub fn event_data_msg_rate_threshold(&self) -> Option<&::models::EventThresholdByValue> {
    self.event_data_msg_rate_threshold.as_ref()
  }

  pub fn reset_event_data_msg_rate_threshold(&mut self) {
    self.event_data_msg_rate_threshold = None;
  }

  pub fn set_event_max_memory_threshold(&mut self, event_max_memory_threshold: ::models::EventThresholdByPercent) {
    self.event_max_memory_threshold = Some(event_max_memory_threshold);
  }

  pub fn with_event_max_memory_threshold(mut self, event_max_memory_threshold: ::models::EventThresholdByPercent) -> MsgVpnDistributedCacheCluster {
    self.event_max_memory_threshold = Some(event_max_memory_threshold);
    self
  }

  pub fn event_max_memory_threshold(&self) -> Option<&::models::EventThresholdByPercent> {
    self.event_max_memory_threshold.as_ref()
  }

  pub fn reset_event_max_memory_threshold(&mut self) {
    self.event_max_memory_threshold = None;
  }

  pub fn set_event_max_topics_threshold(&mut self, event_max_topics_threshold: ::models::EventThresholdByPercent) {
    self.event_max_topics_threshold = Some(event_max_topics_threshold);
  }

  pub fn with_event_max_topics_threshold(mut self, event_max_topics_threshold: ::models::EventThresholdByPercent) -> MsgVpnDistributedCacheCluster {
    self.event_max_topics_threshold = Some(event_max_topics_threshold);
    self
  }

  pub fn event_max_topics_threshold(&self) -> Option<&::models::EventThresholdByPercent> {
    self.event_max_topics_threshold.as_ref()
  }

  pub fn reset_event_max_topics_threshold(&mut self) {
    self.event_max_topics_threshold = None;
  }

  pub fn set_event_request_queue_depth_threshold(&mut self, event_request_queue_depth_threshold: ::models::EventThresholdByPercent) {
    self.event_request_queue_depth_threshold = Some(event_request_queue_depth_threshold);
  }

  pub fn with_event_request_queue_depth_threshold(mut self, event_request_queue_depth_threshold: ::models::EventThresholdByPercent) -> MsgVpnDistributedCacheCluster {
    self.event_request_queue_depth_threshold = Some(event_request_queue_depth_threshold);
    self
  }

  pub fn event_request_queue_depth_threshold(&self) -> Option<&::models::EventThresholdByPercent> {
    self.event_request_queue_depth_threshold.as_ref()
  }

  pub fn reset_event_request_queue_depth_threshold(&mut self) {
    self.event_request_queue_depth_threshold = None;
  }

  pub fn set_event_request_rate_threshold(&mut self, event_request_rate_threshold: ::models::EventThresholdByValue) {
    self.event_request_rate_threshold = Some(event_request_rate_threshold);
  }

  pub fn with_event_request_rate_threshold(mut self, event_request_rate_threshold: ::models::EventThresholdByValue) -> MsgVpnDistributedCacheCluster {
    self.event_request_rate_threshold = Some(event_request_rate_threshold);
    self
  }

  pub fn event_request_rate_threshold(&self) -> Option<&::models::EventThresholdByValue> {
    self.event_request_rate_threshold.as_ref()
  }

  pub fn reset_event_request_rate_threshold(&mut self) {
    self.event_request_rate_threshold = None;
  }

  pub fn set_event_response_rate_threshold(&mut self, event_response_rate_threshold: ::models::EventThresholdByValue) {
    self.event_response_rate_threshold = Some(event_response_rate_threshold);
  }

  pub fn with_event_response_rate_threshold(mut self, event_response_rate_threshold: ::models::EventThresholdByValue) -> MsgVpnDistributedCacheCluster {
    self.event_response_rate_threshold = Some(event_response_rate_threshold);
    self
  }

  pub fn event_response_rate_threshold(&self) -> Option<&::models::EventThresholdByValue> {
    self.event_response_rate_threshold.as_ref()
  }

  pub fn reset_event_response_rate_threshold(&mut self) {
    self.event_response_rate_threshold = None;
  }

  pub fn set_global_caching_enabled(&mut self, global_caching_enabled: bool) {
    self.global_caching_enabled = Some(global_caching_enabled);
  }

  pub fn with_global_caching_enabled(mut self, global_caching_enabled: bool) -> MsgVpnDistributedCacheCluster {
    self.global_caching_enabled = Some(global_caching_enabled);
    self
  }

  pub fn global_caching_enabled(&self) -> Option<&bool> {
    self.global_caching_enabled.as_ref()
  }

  pub fn reset_global_caching_enabled(&mut self) {
    self.global_caching_enabled = None;
  }

  pub fn set_global_caching_heartbeat(&mut self, global_caching_heartbeat: i64) {
    self.global_caching_heartbeat = Some(global_caching_heartbeat);
  }

  pub fn with_global_caching_heartbeat(mut self, global_caching_heartbeat: i64) -> MsgVpnDistributedCacheCluster {
    self.global_caching_heartbeat = Some(global_caching_heartbeat);
    self
  }

  pub fn global_caching_heartbeat(&self) -> Option<&i64> {
    self.global_caching_heartbeat.as_ref()
  }

  pub fn reset_global_caching_heartbeat(&mut self) {
    self.global_caching_heartbeat = None;
  }

  pub fn set_global_caching_topic_lifetime(&mut self, global_caching_topic_lifetime: i64) {
    self.global_caching_topic_lifetime = Some(global_caching_topic_lifetime);
  }

  pub fn with_global_caching_topic_lifetime(mut self, global_caching_topic_lifetime: i64) -> MsgVpnDistributedCacheCluster {
    self.global_caching_topic_lifetime = Some(global_caching_topic_lifetime);
    self
  }

  pub fn global_caching_topic_lifetime(&self) -> Option<&i64> {
    self.global_caching_topic_lifetime.as_ref()
  }

  pub fn reset_global_caching_topic_lifetime(&mut self) {
    self.global_caching_topic_lifetime = None;
  }

  pub fn set_max_memory(&mut self, max_memory: i64) {
    self.max_memory = Some(max_memory);
  }

  pub fn with_max_memory(mut self, max_memory: i64) -> MsgVpnDistributedCacheCluster {
    self.max_memory = Some(max_memory);
    self
  }

  pub fn max_memory(&self) -> Option<&i64> {
    self.max_memory.as_ref()
  }

  pub fn reset_max_memory(&mut self) {
    self.max_memory = None;
  }

  pub fn set_max_msgs_per_topic(&mut self, max_msgs_per_topic: i64) {
    self.max_msgs_per_topic = Some(max_msgs_per_topic);
  }

  pub fn with_max_msgs_per_topic(mut self, max_msgs_per_topic: i64) -> MsgVpnDistributedCacheCluster {
    self.max_msgs_per_topic = Some(max_msgs_per_topic);
    self
  }

  pub fn max_msgs_per_topic(&self) -> Option<&i64> {
    self.max_msgs_per_topic.as_ref()
  }

  pub fn reset_max_msgs_per_topic(&mut self) {
    self.max_msgs_per_topic = None;
  }

  pub fn set_max_request_queue_depth(&mut self, max_request_queue_depth: i64) {
    self.max_request_queue_depth = Some(max_request_queue_depth);
  }

  pub fn with_max_request_queue_depth(mut self, max_request_queue_depth: i64) -> MsgVpnDistributedCacheCluster {
    self.max_request_queue_depth = Some(max_request_queue_depth);
    self
  }

  pub fn max_request_queue_depth(&self) -> Option<&i64> {
    self.max_request_queue_depth.as_ref()
  }

  pub fn reset_max_request_queue_depth(&mut self) {
    self.max_request_queue_depth = None;
  }

  pub fn set_max_topic_count(&mut self, max_topic_count: i64) {
    self.max_topic_count = Some(max_topic_count);
  }

  pub fn with_max_topic_count(mut self, max_topic_count: i64) -> MsgVpnDistributedCacheCluster {
    self.max_topic_count = Some(max_topic_count);
    self
  }

  pub fn max_topic_count(&self) -> Option<&i64> {
    self.max_topic_count.as_ref()
  }

  pub fn reset_max_topic_count(&mut self) {
    self.max_topic_count = None;
  }

  pub fn set_msg_lifetime(&mut self, msg_lifetime: i64) {
    self.msg_lifetime = Some(msg_lifetime);
  }

  pub fn with_msg_lifetime(mut self, msg_lifetime: i64) -> MsgVpnDistributedCacheCluster {
    self.msg_lifetime = Some(msg_lifetime);
    self
  }

  pub fn msg_lifetime(&self) -> Option<&i64> {
    self.msg_lifetime.as_ref()
  }

  pub fn reset_msg_lifetime(&mut self) {
    self.msg_lifetime = None;
  }

  pub fn set_msg_vpn_name(&mut self, msg_vpn_name: String) {
    self.msg_vpn_name = Some(msg_vpn_name);
  }

  pub fn with_msg_vpn_name(mut self, msg_vpn_name: String) -> MsgVpnDistributedCacheCluster {
    self.msg_vpn_name = Some(msg_vpn_name);
    self
  }

  pub fn msg_vpn_name(&self) -> Option<&String> {
    self.msg_vpn_name.as_ref()
  }

  pub fn reset_msg_vpn_name(&mut self) {
    self.msg_vpn_name = None;
  }

  pub fn set_new_topic_advertisement_enabled(&mut self, new_topic_advertisement_enabled: bool) {
    self.new_topic_advertisement_enabled = Some(new_topic_advertisement_enabled);
  }

  pub fn with_new_topic_advertisement_enabled(mut self, new_topic_advertisement_enabled: bool) -> MsgVpnDistributedCacheCluster {
    self.new_topic_advertisement_enabled = Some(new_topic_advertisement_enabled);
    self
  }

  pub fn new_topic_advertisement_enabled(&self) -> Option<&bool> {
    self.new_topic_advertisement_enabled.as_ref()
  }

  pub fn reset_new_topic_advertisement_enabled(&mut self) {
    self.new_topic_advertisement_enabled = None;
  }

}



